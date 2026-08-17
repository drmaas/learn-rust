# Week 8 — Smart Pointers, Lifetimes Deep Dive, Memory Model, Unsafe-lite

**Goal:** Look under the hood. This week is where you move from "writes working Rust" toward "understands Rust" — the level needed to read and contribute to core libraries. It's the hardest week so far; go slow.

**Facet:** syntax & internal model (how Rust compiles to native code, how memory works).

---

## Map from your languages

| Rust | Java | Python | C (for reference) |
|------|------|--------|-------------------|
| `Box<T>` | heap object | object | `malloc` + owner |
| `Rc<T>` | — | refcount (GC) | manual refcount |
| `Arc<T>` | — | — | atomic refcount |
| `RefCell<T>` | — | — | manual borrow checking |
| `Drop` | finalizer/GC | `__del__` | `free` |
| raw pointers | — | `ctypes` | `*ptr` |

**The big idea:** Java/Python/TS hide all memory management. Rust makes it *explicit but safe*. Smart pointers are the safe vocabulary for the things C does with raw pointers.

---

## Day 1 (Mon) — `Box`, `Rc`, `Arc`

**Recall (2 min):** What's the difference between `String` and `&str` (owned vs view)? (This is the model for everything today.)

**Theory (10 min):**
- `Box<T>` — a heap-allocated value with a single owner. Needed for **recursive types** (a type that contains itself has infinite size unless one layer is a pointer):
  ```rust
  enum List {
      Cons(i32, Box<List>),
      Nil,
  }
  ```
- `Rc<T>` — **reference-counted** shared ownership (single-threaded). Multiple owners; the value is dropped when the last `Rc` is dropped. `Rc::clone(&x)` increments the count (cheap — it's not a deep clone).
- `Arc<T>` — atomic `Rc`, safe to share across threads (Week 9).
- Reference cycles leak (two `Rc`s pointing at each other) — `Weak<T>` breaks cycles.

**Exercise (15 min):** Build the `List` enum above and create `Cons(1, Cons(2, Nil))`. Then share a value with `Rc`:
```rust
use std::rc::Rc;
let a = Rc::new(5);
let b = Rc::clone(&a);
println!("{}", Rc::strong_count(&a));  // 2
```
Predict the count before running. Then build a tiny tree with `Rc` where two nodes share a child.

**Reflect (3 min):** Why can't `enum List { Cons(i32, List) }` exist? (Infinite size — `List` would contain a `List`... forever. `Box` adds one pointer of indirection.)

---

## Day 2 (Tue) — `RefCell` and interior mutability

**Recall (2 min):** When do you use `Arc` instead of `Rc`?

**Theory (10 min):**
- The borrow rules (`&` or `&mut`) are normally enforced at **compile time**. `RefCell<T>` moves that check to **runtime** — it *is* the borrow checker, but on the clock.
- `borrow()` gives `Ref<T>` (panics if already mutably borrowed); `borrow_mut()` gives `RefMut<T>` (panics if borrowed at all).
- **Why it exists:** sometimes you need shared data that changes — e.g., a graph where nodes point at each other. `Rc<RefCell<T>>` is the classic combo: shared ownership *and* mutable.
- Tradeoff: runtime panics instead of compile-time errors; single-threaded only.
- `Mutex<T>` is the thread-safe version (Week 9).

**Exercise (15 min):** Build a small graph/network where each node is `Rc<RefCell<Node>>`, with `Node { name: String, neighbors: Vec<Rc<RefCell<Node>>> }`. Add an edge, mutate a node's name through the shared reference, and count nodes. Then deliberately cause a `borrow()` panic (borrow twice) and read the panic message — it tells you exactly where.

**Reflect (3 min):** Why does Rust offer `RefCell` (runtime checking) when the compiler can check borrows for free? (Because some patterns — shared mutable graphs — can't be expressed with compile-time rules. Rust gives you a safe escape hatch *with a panic instead of memory corruption*.)

---

## Day 3 (Wed) — Lifetimes deep dive

**Recall (2 min):** What does `Rc<RefCell<T>>` give you?

**Theory (10 min):**
- **Lifetime elision rules** (when you can omit `'a`): every input reference gets its own lifetime; if there's exactly one input lifetime, it's assigned to all output references; if `&self`, its lifetime is assigned to outputs.
- Explicit lifetimes: `fn longest<'a>(x: &'a str, y: &'a str) -> &'a str`.
- Structs can hold references — but then the struct needs a lifetime parameter:
  ```rust
  struct Slice<'a> {
      data: &'a [u8],
  }
  ```
- `'static` — the reference lives for the whole program (string literals are `&'static str`). It appears in error messages a lot; it means "you returned a reference that isn't tied to the input."
- **Variance** (just know it exists): `&'a T` is covariant in `'a` — you can shrink a lifetime. Don't memorize the rules; the compiler will teach you.

**Exercise (15 min):** (1) Write `longest` with explicit lifetimes (from Week 2) and then *call* it in a way that fails: create two `String`s, take `longest` of them, then drop one before using the result — read the error. (2) Write a `struct BookRef<'a> { title: &'a str }` and use it. (3) Write a function returning `&'static str` (a literal) and one that tries to return a reference to a local — read how the error mentions `'static`.

**Reflect (3 min):** Why does a struct holding a reference need a lifetime parameter but a struct holding a `String` doesn't? (The struct must know how long the borrowed data stays valid. Owned data doesn't borrow anything.)

---

## Day 4 (Thu) — Memory layout and `Drop`

**Recall (2 min):** What is `'static`?

**Theory (10 min):**
- `size_of::<T>()`, `align_of::<T>()` — Rust lets you inspect memory layout.
- `#[repr(C)]` — C-compatible layout (needed for FFI, Week 10). Default `repr(Rust)` layout is unspecified (so the compiler can optimize).
- **Enums are tagged:** `enum E { A, B(u64) }` stores a discriminant + payload; the size is the max payload + tag (with niche optimizations: `Option<&T>` is the same size as `&T` — the null pointer is the niche).
- `Drop` — run code when a value is dropped (RAII). This is how `File`, `MutexGuard`, and smart pointers auto-cleanup:
  ```rust
  struct Guard;
  impl Drop for Guard {
      fn drop(&mut self) { println!("cleaning up!"); }
  }
  ```
- Drop order: fields drop in declaration order; values drop in reverse scope order (LIFO).

**Exercise (15 min):** Print `size_of`/`align_of` for `u8, u32, f64, bool, String, Vec<u8>, Option<u8>, Option<&u8>, Box<u8>` — predict sizes first, then check (note `Option<&u8>` == `&u8` due to niche optimization). Then implement `Drop` on a couple of structs and observe the drop order with prints. Then `#[repr(C)]` a struct and compare its size to the default.

**Reflect (3 min):** Why does `Option<&T>` take no extra space? (The null pointer is an invalid `&T`, so the compiler uses it as the `None` tag — called a niche optimization. This is why Rust's `Option` is free.)

---

## Day 5 (Fri) — Practice: lifetimes + memory

**Recall (2 min):** What's a niche optimization?

**Exercise (25 min):**
1. Do the Rustlings `lifetimes` exercises (there are ~5).
2. Build a small **custom smart-pointer-ish** type: `struct MyBox<T>(T)` with a `new`, a `deref`-like `get(&self) -> &T`, and a `Drop` impl that prints. See how much work `Box` does for you.
3. **Draw** the memory diagram for this program (stack frames, heap, who owns what):
   ```rust
   fn main() {
       let s = String::from("hello");
       let r = &s;
       let b = Box::new(42);
       println!("{} {}", r, b);
   }
   ```

**Reflect (3 min):** Which of today's/this week's concepts would you struggle to explain to a Java developer? Write that explanation anyway — that's the Feynman technique.

---

## Weekend checkpoint (~15 min)

**Take the quiz:** [Week 8 quiz](../quizzes/week-08.md) — closed-book, then check the answers. Mark your misses in the quiz file.

1. From memory: (a) `Box` vs `Rc` vs `Arc`; (b) why `RefCell` exists; (c) lifetime elision rule #1; (d) what `Drop` is for.
2. Redo the `Rc<RefCell<Node>>` graph from Day 2 from scratch.
3. Update `progress.md` (this week's errors are worth logging).

## Reading assignment (10 min)

The Book **chapter 15** (smart pointers) and **19** (advanced: lifetimes, unsafe). Then open the docs for `Box` (`https://doc.rust-lang.org/std/boxed/struct.Box.html`) and skim. If you're feeling brave, open a small slice of a real crate that uses `unsafe` — e.g., `libc` or the Linux parts of `wry` — and just *look* at what `unsafe` blocks surround. You'll formalize this in Week 10.

## Resources

- The Book, ch. 15, 19
- Rustlings: `smart_pointers`, `lifetimes`
- Crust of Rust: "Lifetimes" (video) — the best deep dive that exists
