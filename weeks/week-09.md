# Week 9 — Concurrency & Async

**Goal:** Write safe concurrent and asynchronous code. This is where Rust's design pays off — the same ownership rules that confused you in Week 2 now *guarantee* your multithreaded code has no data races. And async is how Tauri's commands work.

**Facet:** paradigm + ecosystem (tokio).

---

## Map from your languages

| Rust | Java | Python | TypeScript |
|------|------|--------|-----------|
| `std::thread` | `Thread` | `threading` | web workers |
| `mpsc` channel | `BlockingQueue` | `queue.Queue` | — |
| `Arc<Mutex<T>>` | `synchronized`/locks | `threading.Lock` | — |
| `Send`/`Sync` | — | — | — |
| `async`/`await` | `CompletableFuture` | `asyncio` | `async`/`await`/promises |
| tokio | executor/fork-join | `asyncio` loop | event loop |

**The key idea:** `Send` and `Sync` are marker traits the compiler checks automatically — a type is `Send` if it's safe to move to another thread, `Sync` if it's safe to share by reference. The borrow checker + these markers mean **data races are compile-time errors in Rust**, a guarantee no other mainstream language gives you.

---

## Day 1 (Mon) — Threads

**Recall (2 min):** What does `move` do for closures? (You'll need it today.)

**Theory (10 min):**
```rust
let handle = std::thread::spawn(|| {
    println!("in thread!");
});
handle.join().unwrap();   // wait for it
```
- `thread::spawn` takes a closure; the closure must be `Send` + `'static`.
- To use data from the main thread, `move` it in: `thread::spawn(move || println!("{data}"))`.
- `JoinHandle::join()` waits and returns the thread's `Result`.
- `thread::spawn` is **1:1 OS threads** — real parallelism (Python: GIL means threads don't parallelize CPU work; Rust has no GIL).
- `thread::scope` — scoped threads that can borrow non-`'static` data (a nice modern alternative).

**Exercise (15 min):** Spawn 8 threads that each compute a chunk of work (e.g., sum 1..=N for their slice) and print `thread {i} done`. Join them all. Then use `move` to give each thread its own value from a `Vec` and print it — predict what order they print in (it won't be insertion order).

**Reflect (3 min):** Why must the closure be `'static` (own its data)? (The thread can outlive the current scope, so it can't borrow data that might be dropped.)

---

## Day 2 (Tue) — Channels and shared state

**Recall (2 min):** What does `join()` do?

**Theory (10 min):**
- **Channels** (message passing): `mpsc` = multiple producer, single consumer.
  ```rust
  use std::sync::mpsc;
  let (tx, rx) = mpsc::channel();
  let tx2 = tx.clone();                       // multiple producers
  std::thread::spawn(move || tx.send(1).unwrap());
  std::thread::spawn(move || tx2.send(2).unwrap());
  for msg in rx { println!("{msg}"); }        // consumes until all senders drop
  ```
- **Shared state:** `Arc<Mutex<T>>` — `Arc` shares ownership across threads, `Mutex` gives safe mutable access (lock/unlock).
  ```rust
  let counter = Arc::new(Mutex::new(0));
  let c = Arc::clone(&counter);
  std::thread::spawn(move || {
      let mut n = c.lock().unwrap();
      *n += 1;
  });
  ```
- `MutexGuard` auto-unlocks on drop (RAII from Week 8!).
- **The Rust guarantee:** you literally *cannot* share a `T` across threads unless it's `Send`/`Sync`. `Rc<T>` is not `Send` — trying to use it in a thread is a compile error. Java's `synchronized` is a runtime warning at best.

**Exercise (15 min):** Build a producer/consumer pipeline: 4 worker threads each produce results (e.g., squared numbers or simulated work), send them on a shared `tx`, and the main thread collects via `rx`. Then do the same job with `Arc<Mutex<Vec<u32>>>` shared state instead of a channel. Try to use `Rc` instead of `Arc` and read the "not Send" error — that error is the whole point of this week.

**Reflect (3 min):** Channels vs shared state — when would you pick each? (Channels: pipelines, decoupled stages, ownership moves cleanly. Shared state: when multiple threads need the same changing data.)

---

## Day 3 (Wed) — async/await fundamentals

**Recall (2 min):** Why can't `Rc<T>` cross threads?

**Theory (10 min):**
- **Threads vs async:** threads are OS-scheduled, stack-per-thread, great for CPU-bound parallel work. Async lets *one* thread juggle many *I/O-bound* tasks (waiting on network/disk) without blocking — like Python's asyncio or JS promises.
- A `Future` is a value that will produce a result later. `async fn` returns a `Future`.
- `.await` suspends the current task until the future completes — without blocking the thread.
- **Nothing runs until you drive the future** — you need an executor. `futures::executor::block_on` is the simplest; tokio is the real one.
- `async fn` + `.await` composes: `let r = fetch_stuff().await;`

**Exercise (15 min):**
```rust
async fn add(a: u32, b: u32) -> u32 { a + b }

fn main() {
    let fut = add(2, 3);            // nothing has run yet — it's a Future
    let result = futures::executor::block_on(fut);
    println!("{result}");
}
```
Add `futures` (`cargo add futures`). Predict what happens if you *don't* `block_on`. Then write two async functions that call each other with `.await` and chain them.

**Reflect (3 min):** What's the difference between `.await` and blocking? (`.await` yields the thread to other tasks while waiting; blocking holds the thread hostage.)

---

## Day 4 (Thu) — tokio

**Recall (2 min):** What does an executor do?

**Theory (10 min):**
- **tokio** is the de-facto async runtime (Tauri uses it). `#[tokio::main]` sets up the runtime for you.
- `tokio::spawn` — spawn a task (like a lightweight thread, multiplexed onto the runtime's threads).
- `tokio::sync::mpsc` / `oneshot` — async channels.
- `select!` — wait on multiple futures, run whichever completes first.
- `tokio::time::sleep` — non-blocking sleep (unlike `std::thread::sleep`).
- **CPU-bound work should still use threads** (`tokio::task::spawn_blocking`); async is for I/O.

**Exercise (15 min):**
```rust
#[tokio::main]
async fn main() {
    let mut handles = vec![];
    for i in 0..10 {
        handles.push(tokio::spawn(async move {
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            i * i
        }));
    }
    for h in handles {
        println!("{}", h.await.unwrap());
    }
}
```
Then write a `select!` that races two futures (e.g., a sleep and a computation) and prints whichever wins. Add `tokio` with `cargo add tokio --features full`.

**Reflect (3 min):** Tasks vs threads — what's the practical difference? (Tasks are cheap, scheduler-managed, scale to thousands; threads are OS objects with their own stacks.)

---

## Day 5 (Fri) — Practice: parallel word counter

**Recall (2 min):** What does `#[tokio::main]` do?

**Exercise (25 min):** Upgrade your Week 4 `word_count` CLI to be parallel:
1. Create 4–6 text files (or reuse one big one split into parts).
2. **Threads version:** spawn one thread per file, each counts words and sends `(filename, count)` over a channel; main thread sums.
3. **Async version:** with tokio, read all files concurrently (`tokio::fs::read_to_string`) and sum.
4. Compare the single-threaded vs parallel timings on a big file (generate one with a loop).

Run `clippy --pedantic`. Note: `tokio::fs` needs the `fs` feature.

**Reflect (3 min):** Which version felt more natural? When would you reach for threads vs async in a real tool?

---

## Weekend checkpoint (~15 min)

**Take the quiz:** [Week 9 quiz](../quizzes/week-09.md) — closed-book, then check the answers. Mark your misses in the quiz file.

1. From memory: (a) `Send` vs `Sync`; (b) channels vs `Arc<Mutex<T>>`; (c) what `.await` does; (d) tasks vs threads.
2. Redo the producer/consumer pipeline from Day 2 from scratch.
3. Update `progress.md`.

## Reading assignment (10 min)

The Book **chapter 16** (concurrency). Then the Async Book's first chapters (`https://rust-lang.github.io/async-book/`). Finally, read Tauri's "Calling Rust from the Frontend" page (`https://v2.tauri.app/develop/calling-rust/`) — note how async commands are written; you'll write these in Weeks 11–12.

## Resources

- The Book, ch. 16
- Async Book: https://rust-lang.github.io/async-book/
- Rustlings: `concurrency` (threads, channels)
- tokio tutorial: https://tokio.rs/tokio/tutorial
