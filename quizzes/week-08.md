# Week 8 Quiz — Smart Pointers, Lifetimes, Memory, Unsafe-lite

**Missed questions (fill in as you go):**

---

## Questions (answer closed-book, in writing)

1. `Box<T>`, `Rc<T>`, `Arc<T>` — when do you use each?

2. Why do recursive types (like a linked list) need a `Box`?

3. What is `RefCell<T>`, and why does it exist when the borrow checker already checks borrows?

4. What is the `Drop` trait for? Give one example of RAII in the standard library.

5. Why does `fn first_word(s: &str) -> &str` compile *without* an explicit lifetime, while `fn longest(x: &str, y: &str) -> &str` doesn't?

6. What does `'static` mean?

7. Why is `Option<&T>` the **same size** as `&T`?

8. **Predict:** which is bigger, `size_of::<u8>()` or `size_of::<Option<u8>>()`?

9. What is `#[repr(C)]` for, and when do you need it?

10. In what order are a struct's fields dropped? In what order are variables in a scope dropped?

11. **Does this compile? Why or why not?**
   ```rust
   use std::rc::Rc;
   fn main() {
       let a = Rc::new(5);
       let b = Rc::clone(&a);
       println!("{}", Rc::strong_count(&a));
   }
   ```

12. **Map from your languages:** Java's `ArrayList` is a heap object you don't think about. In Rust, `Vec<T>`'s data lives on the heap too — what makes the *ownership* explicit?

---

## Answers & Explanations

1. **`Box<T>`** — single owner, heap-allocated; use for recursive types or when you want the heap explicitly. **`Rc<T>`** — reference-counted *shared* ownership, **single-threaded**; the value lives as long as any `Rc` points to it. **`Arc<T>`** — the **atomic** version, safe to share across threads. `Box` = one owner; `Rc`/`Arc` = many owners.

2. Because a recursive type contains itself — its size would be **infinite**. `enum List { Cons(i32, List) }` can't be sized. `Box<List>` adds one level of indirection (a pointer), breaking the recursion: `enum List { Cons(i32, Box<List>), Nil }` has a finite, known size.

3. `RefCell<T>` moves the borrow check from **compile time to runtime**: `borrow()`/`borrow_mut()` check at runtime and panic on violation. It exists for **interior mutability** — patterns where you need shared, mutable data that the compile-time rules can't express (e.g., a graph where nodes reference each other: `Rc<RefCell<Node>>`). Tradeoff: runtime panic instead of compile error; single-threaded.

4. `Drop` runs code **when a value goes out of scope** — the cleanup is tied to the owner's lifetime (RAII). Standard-library examples: `File` (closes the file), `MutexGuard` (releases the lock), `Box`/`Rc` (free the memory). You never call `close()`/`free()` manually — the type handles it.

5. **Lifetime elision rules:** if a function has exactly **one** input reference, that lifetime is assigned to all output references. `first_word` has one input (`&str`), so the output gets it automatically. `longest` has **two** inputs — the compiler can't know which one is returned, so it demands an explicit `'a`.

6. `'static` means the reference **lives for the entire program**. String literals (`"hello"`) are `&'static str` — they're baked into the binary. It appears in error messages when you return a reference not tied to an input (the compiler suggests "maybe you meant 'static"). Note: `'static` is a *lifetime*, not "the value never changes."

7. **Niche optimization.** A `&T` is never a null pointer, so the compiler uses the all-zeros bit pattern as the `None` tag. `Option<&T>` therefore stores just the pointer — `None` is the null pointer. Free `Option`, no extra byte. The same trick applies to `Option<Box<T>>`, `Option<NonZeroU32>`, etc.

8. **`Option<u8>` is bigger (2 bytes vs 1).** `u8` has no niche — all 256 bit patterns are valid values, so there's no spare pattern to mean `None`; the compiler must add a separate tag byte. (Contrast with `&T` and `NonZero*` types, which have a spare pattern.)

9. `#[repr(C)]` gives a struct a **guaranteed, C-compatible memory layout** (field order and alignment match C). You need it for **FFI** — passing structs to/from C — and for anything where the layout is a contract (e.g., a serialized format). Default `repr(Rust)` layout is unspecified so the compiler can reorder fields for efficiency.

10. A struct's fields drop in **declaration order** (top to bottom). Variables in a scope drop in **reverse declaration order** (LIFO — last declared, first dropped). Drop is deterministic and ordered — a feature GC languages don't give you.

11. **Compiles and prints `2`.** `Rc::clone(&a)` doesn't deep-copy — it increments the reference count. `strong_count(&a)` returns 2 (both `a` and `b` point to the same allocation). When both go out of scope, the count hits 0 and the value is freed.

12. In Java, `ArrayList` is garbage-collected and you never think about ownership. In Rust, `Vec<T>` **owns** its heap buffer, and that ownership is tracked: moving the `Vec` moves the buffer, dropping the `Vec` frees the buffer, and `&Vec`/`&mut Vec` borrow it under the usual rules. The explicit part is that *someone owns it, exactly one person, and it's dropped when they're done* — no GC, no leaks, no double-free.
