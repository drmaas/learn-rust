# Week 4 Quiz — Error Handling

**Missed questions (fill in as you go):**

---

## Questions (answer closed-book, in writing)

1. What's the fundamental difference between Rust's `Result` and Java/Python exceptions?

2. What does the `?` operator do? What must the enclosing function return to use it?

3. When should you `panic!` vs return `Result`? Give one example of each.

4. What two traits must a custom error type implement (beyond `Debug`)?

5. **Predict the output:**
   ```rust
   fn parse(s: &str) -> Result<i32, std::num::ParseIntError> {
       s.trim().parse()
   }
   fn main() {
       let a = parse("42").unwrap_or(0);
       let b = parse("not a number").unwrap_or(0);
       println!("{} {}", a, b);
   }
   ```

6. What does `ok_or` do? What types does it convert between?

7. `map` vs `and_then` on `Option`/`Result` — what's the difference?

8. Why is `expect("message")` better than `unwrap()`?

9. **Write code:** a function `divide(a: f64, b: f64) -> Result<f64, String>` that returns an error for `b == 0.0`.

10. Why does `?` on a `std::io::Error` work inside a function returning your custom `ConfigError`?

11. **Map from your languages:** in Python you'd write `try: x = int(s) except ValueError: x = 0`. Write the Rust equivalent that never panics.

12. `main` can return `Result<(), Box<dyn std::error::Error>>`. What does that let you do, and what's the downside?

---

## Answers & Explanations

1. Exceptions are **thrown** and fly up the call stack invisibly — the function signature doesn't say it can fail, and callers can forget to handle it. `Result` is a **return value** — failure is part of the type. The compiler forces you to handle it (or explicitly defer with `?`). You literally cannot forget to handle an error.

2. `x?` unwraps `Ok(x)` or **returns early** with the `Err` — but it must be *converted* to the function's error type (via `From`). The enclosing function must return `Result` (or `Option`). It's like "throw," but explicit and type-checked.

3. **Panic** when the failure is a *programmer error or broken invariant* — e.g., indexing past the end of a fixed array, or `assert!(cfg.is_valid())` in a debug build. **`Result`** when failure is a *possible, expected outcome* — e.g., file not found, invalid user input, network error. Rule of thumb: if the caller could reasonably recover, return `Result`.

4. **`Display`** (so it can be printed with `{}` and shown to users) and **`std::error::Error`** (so it's a first-class error that `?`, `anyhow`, and logging can use). `Debug` is also conventional (derive it).

5. **`42 0`.** `parse("42")` → `Ok(42)` → `unwrap_or(0)` gives 42. `parse("not a number")` → `Err` → `unwrap_or(0)` gives 0. No panic — `unwrap_or` handles the failure gracefully.

6. `ok_or` converts **`Option<T>` → `Result<T, E>`**: `Some(v)` becomes `Ok(v)`, `None` becomes `Err(e)` where you supply `e`. Use it when "missing value" should be reported as an error with a message.

7. `map` transforms the *success* value: `Option<T>` → `Option<U>` (`Some(1).map(|x| x + 1)` = `Some(2)`). `and_then` chains an operation that itself returns an `Option`/`Result` (`Some("4").and_then(|s| s.parse().ok())` = `Some(4)`). Use `map` for plain transforms, `and_then` when the next step can also fail.

8. `expect("msg")` documents *why* the unwrap is safe: `.expect("we checked the length first")`. If it panics, the message tells you which invariant was violated. `unwrap()` gives you a bare panic with no context. In production code, prefer `?` or real handling; when you must unwrap, `expect` with a reason.

9.
   ```rust
   fn divide(a: f64, b: f64) -> Result<f64, String> {
       if b == 0.0 {
           Err("division by zero".to_string())
       } else {
           Ok(a / b)
       }
   }
   ```

10. Because `?` uses the **`From` trait**: if your `ConfigError` has a variant `Io(std::io::Error)` and you implement `From<std::io::Error> for ConfigError` (or use `thiserror`'s `#[from]`), then `?` on an `io::Result` converts the `io::Error` into your `ConfigError` automatically. The conversion is part of the type system.

11.
   ```rust
   let x = s.parse::<i32>().unwrap_or(0);
   ```
   (Or with `match`/`ok_or` if you need the error. `parse` returns `Result`, so `unwrap_or(0)` gives the default on failure — no exception handling needed.)

12. It lets you use `?` directly in `main` — errors bubble up and `main` returns them (printed to stderr, non-zero exit). Downside: you lose control over *how* the error is presented (no friendly message, no context), and `Box<dyn Error>` erases the concrete error type. Fine for small programs; for real apps, have `main` `match` on a `run() -> Result<(), CliError>` and print a nice message.
