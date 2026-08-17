# Week 10 — FFI, Unsafe, and Systems Programming

**Goal:** Learn Rust's escape hatches — `unsafe` and FFI — and *why* they exist. This is the "core library contributor" skill set: Tauri's `wry` and `tao` crates are full of this stuff. The goal is not to write lots of `unsafe`; it's to understand it well enough to read it, justify it, and contain it.

**Facet:** syntax & internal model.

---

## Map from your languages

| Rust | Java | Python | TypeScript |
|------|------|--------|-----------|
| `unsafe` blocks | JNI | `ctypes` / C extensions | native addons (napi) |
| raw pointers `*const T` | — | `ctypes` pointers | — |
| `extern "C"` | JNI | `ctypes` | napi |
| `#[repr(C)]` | — | struct packing | — |

**The key idea:** `unsafe` doesn't turn off the borrow checker. It gives you four extra powers (and only these):
1. Dereference raw pointers
2. Call unsafe functions (including FFI)
3. Access/implement mutable statics
4. Implement unsafe traits (like `Send`/`Sync`)

Everything else — ownership, borrows, safety — still applies. `unsafe` code must uphold invariants the compiler *can't* check, and the convention is to document them in `// SAFETY:` comments.

---

## Day 1 (Mon) — Unsafe fundamentals

**Recall (2 min):** What's the difference between `&T` and `&mut T`? (You're about to see the raw versions.)

**Theory (10 min):**
- Raw pointers: `*const T` (immutable) and `*mut T` (mutable). They don't follow the borrow rules — that's why they're unsafe to dereference.
- Creating a raw pointer is safe; **dereferencing** it is unsafe:
  ```rust
  let mut x = 42;
  let ptr: *mut i32 = &mut x;        // safe to create
  unsafe {
      *ptr = 100;                     // unsafe to dereference
  }
  println!("{x}");
  ```
- `ptr::null()`, `ptr::add(n)` / `offset(n)` (pointer arithmetic — like C).
- **Invariants you must uphold:** non-null, aligned, in-bounds, no aliasing violations, valid for the lifetime you use it.
- The four powers list above. Nothing else is unsafe.

**Exercise (15 min):** Write the example above. Then allocate a `Vec<i32>`, get a raw pointer to its data, and read/write elements through the pointer with `add`. Then deliberately create a null pointer and dereference it — read the crash (segfault or panic). Then write a small function with a raw pointer and add a `// SAFETY:` comment explaining the invariants.

**Reflect (3 min):** Why does Rust let you create raw pointers safely but not dereference them? (The creation is harmless; the dereference is where memory-safety violations happen.)

---

## Day 2 (Tue) — FFI: calling C from Rust

**Recall (2 min):** What are the four things `unsafe` lets you do?

**Theory (10 min):**
- **FFI** — Foreign Function Interface: calling code in other languages (usually C) that uses the platform ABI.
- `extern "C" { ... }` declares foreign functions:
  ```rust
  extern "C" {
      fn getpid() -> i32;          // from libc (linked by default on Unix)
      fn strlen(s: *const u8) -> usize;
  }
  ```
- Calling them is `unsafe` (the C side doesn't follow Rust's rules).
- **Strings across the boundary:** C strings are null-terminated `*const c_char`. Rust's `CString` (owned, NUL-terminated) and `CStr` (borrowed) handle conversion:
  ```rust
  use std::ffi::CString;
  let c = CString::new("hello").unwrap();   // fails if it contains NUL
  let ptr = c.as_ptr();                     // *const c_char
  ```
- The **`libc` crate** (`cargo add libc`) provides typed bindings to standard C functions.

**Exercise (15 min):**
```rust
extern "C" {
    fn getpid() -> i32;
    fn strlen(s: *const u8) -> usize;
}
fn main() {
    let pid = unsafe { getpid() };
    println!("pid: {pid}");
    let s = b"hello\0";
    let len = unsafe { strlen(s.as_ptr()) };
    println!("len: {len}");
}
```
Then add the `libc` crate and call `libc::getpid()` through it. Write a function that takes a `&str`, converts to `CString`, and calls a C function.

**Reflect (3 min):** Why is calling `getpid()` unsafe? (It's a foreign function — Rust can't verify the C code follows its rules. Also, in general, FFI calls can have side effects Rust doesn't know about.)

---

## Day 3 (Wed) — FFI: being called *from* C

**Recall (2 min):** What's a `CString` for?

**Theory (10 min):** The other direction — exposing Rust to C:
- `#[no_mangle]` — don't mangle the symbol name (so C can link to `my_function`).
- `extern "C" fn` — C calling convention.
- `#[repr(C)]` structs — guaranteed layout (Week 8).
- Build a **`cdylib`** (a C dynamic library) instead of a Rust rlib:
  ```toml
  [lib]
  crate-type = ["cdylib"]
  ```
- `Option<&T>` maps to nullable pointers (None → NULL).
- **ABI stability:** once you publish an `extern "C"` API, its layout is frozen — that's why `#[repr(C)]` matters.

**Exercise (15 min):** Create a crate with:
```rust
#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 { a + b }

#[no_mangle]
pub extern "C" fn greet(name: *const std::os::raw::c_char) -> *const std::os::raw::c_char {
    // convert, build a String, leak it (or return a static) — think about ownership!
    std::ffi::CString::new("hi").unwrap().into_raw()
}
```
Build it as a cdylib. Then call it from **Python via ctypes** (you know Python!):
```python
import ctypes
lib = ctypes.CDLL("./target/debug/libmylib.so")
print(lib.add(2, 3))
```
Note the ownership question in `greet`: who frees the returned string? (This is exactly the kind of thing core-library maintainers think about.)

**Reflect (3 min):** What does `#[no_mangle]` do and why is it needed for C to call Rust?

---

## Day 4 (Thu) — Reading real systems code

**Recall (2 min):** What's a `cdylib`?

**Theory/Exercise (20 min):** Now read real `unsafe` code with fresh eyes. Options (pick one):
1. **wry** (the webview crate Tauri uses) — find its Linux implementation and look for `unsafe` blocks; read the `// SAFETY:` comments.
2. **ripgrep** — search for `unsafe`; see how little there is (ripgrep is famously almost-fully-safe).
3. **libc** or **nix** — raw FFI bindings.

For each `unsafe` block you find, ask: *which of the four powers is it using? What invariant is it upholding? Could it be avoided?* Write a short "reading report" in `progress.md`: 3 examples of `unsafe` you found, what each does, and why it's justified.

**Reflect (3 min):** After reading real code — is `unsafe` common or rare in good Rust codebases? (Rare, small, and documented. That's the ideal.)

---

## Day 5 (Fri) — Practice: a small FFI binding with a safe wrapper

**Recall (2 min):** What are the four powers of `unsafe`?

**Exercise (25 min):** Build a tiny safe wrapper around a C function. Pick something simple — e.g., wrap `getpid`, or `strlen`, or a math function from `libm`:
1. Declare the FFI function in an `extern "C"` block.
2. Write a **safe** Rust function that wraps it, handling all the conversion and error cases *inside*:
   ```rust
   /// Returns the process id. Safe wrapper around libc's getpid.
   pub fn current_pid() -> u32 {
       unsafe { libc::getpid() as u32 }
   }
   ```
3. For a string-based function, handle the NUL-byte case (`CString::new` returns `Result`).
4. Write unit tests for the safe wrapper (including the error path).
5. Run `clippy --pedantic`. Your `unsafe` should be confined to one small function per binding — that's the design goal.

**Reflect (3 min):** What's the "safe wrapper" pattern, and why is it the recommended way to do FFI? (All `unsafe` is contained in tiny, reviewed functions; callers get a safe API. This is how `libc`, `nix`, and Tauri's FFI are designed.)

---

## Weekend checkpoint (~15 min)

**Take the quiz:** [Week 10 quiz](../quizzes/week-10.md) — closed-book, then check the answers. Mark your misses in the quiz file.

1. From memory: (a) the four powers of `unsafe`; (b) `CString` vs `CStr`; (c) `#[no_mangle]` + `#[repr(C)]` — why; (d) the safe-wrapper pattern.
2. Redo the Python-ctypes call from Day 3 from scratch.
3. Update `progress.md` + error log.

## Reading assignment (10 min)

The Book **chapter 19** (unsafe, FFI) and skim the **Rustonomicon** (`https://doc.rust-lang.org/nomicon/`) — just the intro; it's the "unsafe Rust" reference. Then look at Tauri's `ARCHITECTURE.md` on GitHub and note where `wry`/`tao` (the unsafe-heavy crates) sit in the stack. You now understand what they do.

## Resources

- The Book, ch. 19
- The Rustonomicon: https://doc.rust-lang.org/nomicon/
- Crust of Rust: "Unsafe" (video)
- `libc` crate docs
