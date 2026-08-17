# Week 10 Quiz — FFI, Unsafe, Systems Programming

**Missed questions (fill in as you go):**

---

## Questions (answer closed-book, in writing)

1. What are the **four powers** of `unsafe`? (What does it let you do that safe Rust can't?)

2. Why is *dereferencing* a raw pointer unsafe, when *creating* one is safe?

3. `CString` vs `CStr` — which is owned, and which is borrowed?

4. What does `#[no_mangle]` do, and why is it needed for C to call Rust?

5. What does `#[repr(C)]` do, and why is it essential for FFI?

6. What is a **`cdylib`**?

7. What is the **safe-wrapper pattern** for FFI?

8. **Predict the output:**
   ```rust
   let mut x = 5;
   let p: *mut i32 = &mut x;
   unsafe { *p = 10; }
   println!("{x}");
   ```

9. True or false: inside an `unsafe` block, the borrow checker is disabled.

10. Who is responsible for freeing memory returned from an `extern "C"` function, and how do you know?

11. **Write code:** declare an FFI function `fn getpid() -> i32` and call it safely from a `main`.

12. **Map from your languages:** Python's `ctypes` and Java's JNI both let you call C code. What does Rust's FFI require that makes it *safer* than both?

---

## Answers & Explanations

1. `unsafe` gives you exactly four powers (and nothing else): **(1)** dereference raw pointers, **(2)** call unsafe functions (including FFI), **(3)** access or modify mutable statics, **(4)** implement unsafe traits (like `Send`/`Sync`). Everything else — ownership, borrows, safety — still applies inside `unsafe`. It doesn't "turn off" the borrow checker.

2. Creating a pointer (`&mut x` as `*mut i32`) is harmless — the pointer itself is just an address. **Dereferencing** it is where memory safety can break: the pointer might be null, dangling, misaligned, or aliased in a way the compiler can't verify. The `unsafe` block is you saying "I've checked the invariants; trust me here."

3. **`CString`** is **owned** — a heap-allocated, NUL-terminated C string (you build it with `CString::new(...)`, which errors if the input contains a NUL byte). **`CStr`** is **borrowed** — a view into a NUL-terminated C string (like `&str` for C strings). Ownership mirrors `String` vs `&str`.

4. `#[no_mangle]` prevents the Rust compiler from **mangling the symbol name** (Rust adds type info to names, like C++). C code links by plain name — `my_function` must be literally `my_function` in the binary. With `#[no_mangle] pub extern "C" fn`, C can `extern "C"`-declare and call it directly.

5. `#[repr(C)]` gives a struct a **guaranteed, C-compatible layout** — exact field order and alignment matching what C expects. Rust's default layout is unspecified (the compiler may reorder fields). If you pass a struct to C with the wrong layout, you get memory corruption. `repr(C)` makes the layout a stable contract.

6. A **`cdylib`** is a C-compatible **dynamic library** (`.so` on Linux, `.dll` on Windows, `.dylib` on macOS) — compiled from Rust but linkable from C, Python (`ctypes`), or any language with FFI. Set with `[lib] crate-type = ["cdylib"]` in `Cargo.toml`. It's how you ship Rust code as a reusable native library.

7. The safe-wrapper pattern: **confine all `unsafe` to small, well-reviewed functions**, and expose a **safe API** to the rest of your code. Each wrapper handles the conversion and error cases (NUL bytes, null pointers, ownership) *inside*, so callers never touch `unsafe`. This is how `libc`, `nix`, and Tauri's FFI are designed — `unsafe` is isolated, documented, and testable.

8. **`10`.** The raw pointer `p` is created safely (`&mut x` as `*mut i32`), dereferenced inside `unsafe`, writes `10` into `x`. The `unsafe` block is justified and correct here — no dangling, no aliasing violation. (Note: creating the pointer while `x` is still owned is fine; the borrow checker still governs `x`'s normal use.)

9. **False.** `unsafe` gives you the four powers; it does **not** disable the borrow checker. You still can't have a `&` and a `&mut` to the same value in safe code, even inside `unsafe`. The danger is that *raw pointers* bypass the rules — so `unsafe` code must uphold them manually, which is why `// SAFETY:` comments are the convention.

10. **It depends on the function's contract — and that's the point.** Some FFI functions return pointers to static/global data (don't free), some return freshly allocated memory (caller must free, usually with the matching `free` function). You must read the C docs and **document the ownership in the wrapper**'s safety comment. Getting this wrong = memory leak or use-after-free. This is exactly the kind of thing core-library maintainers obsess over.

11.
   ```rust
   extern "C" {
       fn getpid() -> i32;
   }
   fn main() {
       let pid = unsafe { getpid() };
       println!("pid: {pid}");
   }
   ```
   (On Unix, `getpid` is in libc, which Rust links by default. A more robust version uses the `libc` crate: `libc::getpid()`.)

12. Python's `ctypes` and Java's JNI let you call C, but the **safety burden is entirely on you at runtime** — a bad pointer crashes the interpreter/JVM, and there's no compile-time checking. Rust's FFI is safer because: **(1)** the `unsafe` boundary is explicit and local (you know exactly where foreign calls happen), **(2)** the borrow checker still protects all *safe* code around the boundary, **(3)** smart pointers and `CString`/`CStr` handle the dangerous conversions, and **(4)** the type system documents contracts (e.g., `Option<&T>` for nullable pointers). Rust doesn't make FFI safe — it makes the unsafe parts *visible and containable*.
