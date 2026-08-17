# Week 3 Quiz — Structs, Enums, Pattern Matching

**Missed questions (fill in as you go):**

---

## Questions (answer closed-book, in writing)

1. When would you use a `struct` vs an `enum`? Give a one-sentence rule of thumb.

2. What does `#[derive(Debug)]` do, and what does `{:?}` print?

3. A method takes `&self`, `&mut self`, or `self`. What does each mean, and what happens to the value when you use `self`?

4. **Does this compile? Why or why not?**
   ```rust
   fn describe(c: Color) -> &'static str {
       match c {
           Color::Red => "hot",
           Color::Green => "calm",
       }
   }
   ```
   (Assume `enum Color { Red, Green, Blue }`.)

5. What is **exhaustiveness**, and why is it valuable?

6. `Option<T>` vs `Result<T, E>` — when do you use each?

7. **Write code:** a newtype `struct UserId(u64)` and a function `fn register(id: UserId) -> String`.

8. What is `if let` for? Write an example using `Option`.

9. **Refutable vs irrefutable:** which kind of pattern is `let x = 5`? Which is `let Some(v) = opt`? Why does the second need `if let`/`match`?

10. When can you derive `Copy` on your own type?

11. **Map from your languages:** TypeScript's `type Shape = { kind: "circle"; radius: number } | { kind: "square"; side: number }`. Write the equivalent Rust `enum Shape` with the same data.

12. **Predict the output:**
   ```rust
   let o: Option<u32> = Some(7);
   let n = match o {
       Some(v) => v * 2,
       None => 0,
   };
   println!("{n}");
   ```

---

## Answers & Explanations

1. A **`struct`** models a thing that has *all* its fields at once ("a Book has a title AND pages"). An **`enum`** models a thing that is *one of* several alternatives ("a Message is Quit OR Move OR Write"). Struct = AND; enum = OR.

2. `#[derive(Debug)]` auto-implements the `Debug` trait so the type can be printed with `{:?}` (and `{:#?}` for pretty-printed). It's like an automatic `toString()` — for programmers, not users. (For user-facing output you implement `Display` manually.)

3. `&self` borrows immutably (read-only; value still owned by caller). `&mut self` borrows mutably (can change fields). `self` **consumes** the value — the method takes ownership and the original is moved (gone). Use `self` when the method transforms/destroys the value (e.g., `into_iter`).

4. **Does not compile** — `E0004: non-exhaustive patterns`. The `match` doesn't cover `Color::Blue`. The compiler refuses to let you forget a case. Add `Color::Blue => "cold"` (or a `_ =>` arm).

5. Exhaustiveness means a `match` must handle **every** possible value. The compiler enforces it. Value: forgetting a case (or adding a new variant later) is a compile error, not a runtime bug. Adding a variant to an enum forces you to update every `match` — the compiler walks you through your whole codebase.

6. `Option<T>` when the question is *"is there a value or not?"* (a missing key, a parse that might fail). `Result<T, E>` when the question is *"did it succeed, and if not, **what went wrong**?"* — you need the error data. `Option` = maybe; `Result` = ok-or-error.

7.
   ```rust
   struct UserId(u64);

   fn register(id: UserId) -> String {
       format!("registered user {}", id.0)
   }
   ```
   The newtype makes `UserId` a distinct type from `u64` — you can't accidentally pass a raw number where a `UserId` is expected.

8. `if let` matches **one** pattern and ignores the rest — shorthand for a single-arm `match`. Example:
   ```rust
   if let Some(port) = env_port {
       println!("using port {port}");
   } else {
       println!("no port set, using default");
   }
   ```

9. `let x = 5` is **irrefutable** — the pattern always matches, so `let` is allowed. `let Some(v) = opt` is **refutable** — it can fail (if `opt` is `None`), so you must use `if let`/`match`/`let else`. Rust's rule: `let` only accepts irrefutable patterns; refutable ones need conditional syntax (the compiler tells you which).

10. You can derive `Copy` only when **every field is `Copy`** (all primitives, or other `Copy` types). If any field is `String`, `Vec`, etc., you can't — it would silently deep-copy heap data. When in doubt, derive `Clone` only.

11.
   ```rust
   enum Shape {
       Circle { radius: f64 },
       Square { side: f64 },
   }
   ```
   Same idea as the TS union, but the compiler knows every possible shape and forces you to handle both in any `match`.

12. **`14`.** `Some(7)` matches the `Some(v)` arm, `v = 7`, prints `7 * 2 = 14`. If `o` were `None`, it would print `0`.
