# Week 9 Quiz — Concurrency & Async

**Missed questions (fill in as you go):**

---

## Questions (answer closed-book, in writing)

1. `Send` vs `Sync` — what does each marker trait guarantee?

2. Why can't an `Rc<T>` be sent to another thread?

3. Channels vs `Arc<Mutex<T>>` — when would you choose each?

4. What does `.await` do? How is it different from blocking?

5. Threads vs async tasks — when do you use each?

6. What does `#[tokio::main]` do?

7. **Predict the output** (order matters — is it deterministic?):
   ```rust
   use std::sync::mpsc;
   use std::thread;
   let (tx, rx) = mpsc::channel();
   for i in 0..3 {
       let tx = tx.clone();
       thread::spawn(move || tx.send(i).unwrap());
   }
   drop(tx);
   for x in rx { println!("{x}"); }
   ```

8. `tokio::spawn` vs `thread::spawn` — what's the practical difference?

9. What does `select!` do?

10. Why does Rust **guarantee** no data races, when Java/Python/C++ don't?

11. **Write code:** spawn a thread that increments a shared counter, using `Arc<Mutex<i32>>` (just the spawn + increment + join).

12. **Map from your languages:** Python's `asyncio` and JS's `async/await` — how is Rust's async similar, and what's the key difference in how it's executed?

---

## Answers & Explanations

1. **`Send`** — safe to *move* to another thread (the value's ownership can cross thread boundaries). **`Sync`** — safe to *share by reference* across threads (`&T` can be used from multiple threads). `Arc<Mutex<T>>` is `Send` + `Sync`; `Rc<T>` is neither. The compiler checks these automatically — you don't opt in, you *can't* violate them.

2. Because `Rc`'s reference count is **not atomic** — two threads incrementing/decrementing it would race and corrupt the count. `Rc` is `!Send` (not sendable across threads). The compiler rejects `thread::spawn(move || ...)` with an `Rc` inside with a "cannot be sent between threads safely" error — that error is the feature working.

3. **Channels** when you want to pass *ownership* of data between stages (producer → consumer pipelines), when decoupling is the goal, or when each piece of data has one destination. **`Arc<Mutex<T>>`** when multiple threads need to access and mutate the *same* shared data over time. Channels move data; mutexes share it.

4. `.await` **suspends the current task** — the thread is freed to run other tasks — and resumes when the future completes. Blocking (`thread::sleep`, `std::fs` calls) **holds the thread hostage** — nothing else can run on it. `.await` is cooperative multitasking; blocking is not. This is why async is great for I/O-heavy workloads.

5. **Threads** for CPU-bound parallel work (real parallelism, one per core) and long-lived background jobs. **Async tasks** for I/O-bound work — thousands of concurrent connections/requests on a handful of threads. In Rust you often combine them: async for I/O, `tokio::task::spawn_blocking` for CPU-heavy pieces.

6. `#[tokio::main]` transforms your `async fn main()` into a regular `main` that **sets up the tokio runtime** and runs your async code on it. Without a runtime, async code never runs (a `Future` does nothing until an executor drives it). It's the equivalent of `block_on` but for the whole program.

7. **Prints `0`, `1`, `2` in *some* order — not guaranteed.** Threads race; the order is nondeterministic (could be `2 0 1`, etc.). The deterministic part: **all three values arrive** and the loop ends. `drop(tx)` closes the original sender so the receiver's `for` loop terminates once all senders are dropped. (Without the `drop`, the loop would hang forever.)

8. `thread::spawn` creates an **OS thread** (1:1, its own stack, real parallelism). `tokio::spawn` creates a **task** — a lightweight unit of work multiplexed onto the runtime's threads (like a green thread). Tasks are far cheaper: you can spawn thousands. Threads are for parallelism; tasks are for concurrency.

9. `select!` waits on **multiple futures at once** and runs the code for whichever **completes first** (cancelling the others). It's how you race operations — e.g., "wait for the request OR a timeout, whichever comes first." Like JS's `Promise.race`, but with `async` blocks and pattern matching.

10. Because the **type system** enforces it: a type that can be shared across threads must be `Send`/`Sync` (checked at compile time), and mutations require a `Mutex`/`Atomic` (which the borrow checker then governs). Java/Python/C++ allow sharing any mutable object freely — the race is possible and only caught at runtime (or never). Rust makes the race **unrepresentable**.

11.
   ```rust
   use std::sync::{Arc, Mutex};
   let counter = Arc::new(Mutex::new(0));
   let c = Arc::clone(&counter);
   let handle = std::thread::spawn(move || {
       let mut n = c.lock().unwrap();
       *n += 1;
   });
   handle.join().unwrap();
   println!("{}", *counter.lock().unwrap());
   ```
   `lock()` returns a guard that auto-unlocks on drop (RAII from Week 8).

12. Similar: `.await` suspends without blocking, tasks are multiplexed, and it's all cooperative. Key difference: **execution model**. Python's asyncio runs on one thread (GIL), and JS on the event loop — true parallelism requires extra work. Rust's tokio runs tasks across **multiple threads** by default, and you get real parallelism *and* the `Send`/`Sync` safety guarantees. Also, Rust's `Future` is lazy and zero-cost — no runtime objects unless you create them.
