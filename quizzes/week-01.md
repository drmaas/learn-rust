# Week 1 Quiz — Syntax, Types, Control Flow, Cargo

**Missed questions (fill in as you go):**

---

## Questions (answer closed-book, in writing)

1. What are the **four facets** of learning a programming language? (Hint: one is syntax, one is paradigm…)

2. What's the difference between `cargo run` and `cargo build`?

3. **Predict the output:**
   ```rust
   let x = 5;
   let x = x + 1;
   {
       let x = x * 2;
       println!("{x}");
   }
   println!("{x}");
   ```

4. What's the difference between **shadowing** and **mutation**?

5. What type does `let x = 5;` infer? What about `let y = 5.0;`?

6. **Write code:** using an `if` *expression*, assign a `&str` to a variable based on a boolean `is_night` (e.g., "dark" or "light"). No ternary allowed — you shouldn't need one.

7. `String` vs `&str` — which is owned and growable, and which is a borrowed view?

8. True or false: `"hello"` in your source code is a `String`.

9. `s.len()` on a `String` containing `"héllo"` — does it count bytes or characters? What's the value?

10. **Predict:** what does `5 / 2` evaluate to? Why?

11. In a `match` expression, why must every case be covered?

12. **Map from your languages:** in TypeScript you might write `const s = "hi"` for a constant string. What's the closest Rust equivalent, and how is it different if you want to *change* the string later?

---

## Answers & Explanations

1. **(1)** Syntax & semantics (how to write code), **(2)** the paradigm (how to *think* — expression-oriented here), **(3)** the artifact ecosystem (building/running/packages: Cargo), **(4)** tooling (`fmt`, `clippy`, rust-analyzer). When something breaks, ask which facet it belongs to.

2. `cargo run` compiles *and* runs the program. `cargo build` only compiles (produces the binary in `target/`). `run` is `build` + execute.

3. **`12` then `6`.** The inner block shadows `x` with `x * 2` (= 12), which only lives inside the block. After the block, the outer `x` is still 6. Shadowing creates a *new* binding; it doesn't mutate the old one.

4. **Shadowing** declares a new binding with the same name — the old value is still there, just hidden. **Mutation** changes the value the binding points to (requires `let mut`). Shadowing is a transformation step; mutation is a change in place.

5. `let x = 5;` infers **`i32`** (the default integer). `let y = 5.0;` infers **`f64`** (the default float).

6.
   ```rust
   let mood = if is_night { "dark" } else { "light" };
   ```
   Both branches must produce the same type. Blocks are expressions — the last expression is the value.

7. **`String`** is owned, growable, heap-allocated. **`&str`** is a borrowed, immutable view into a `String` (or a literal). You can't grow a `&str` — you'd need to build a new `String`.

8. **False.** `"hello"` is a `&'static str` — a string literal baked into the binary. It becomes a `String` only when you call `.to_string()` or `String::from(...)`.

9. **Bytes.** `"héllo"` is 6 bytes (`h`=1, `é`=2, `l`=1, `l`=1, `o`=1) but 5 characters. Use `s.chars().count()` for characters. This differs from Python, where `len` counts characters.

10. **`2`.** Integer division truncates toward zero. `5 / 2 == 2`. For `2.5` you need floats: `5.0 / 2.0`. (Python 3 would give `2.5` here — one of the few places Rust and Python differ.)

11. **Exhaustiveness.** The compiler forces you to handle every possible case — you can't silently forget one. In Java/Python, a forgotten case just falls through at runtime; in Rust it's a compile error. This is a feature, not an annoyance.

12. `const S: &str = "hi"` (or just `let s = "hi"`). To change it later you'd need an *owned, mutable* string: `let mut s = String::from("hi");`. The key difference: TS strings are always immutable and you just reassign; Rust makes you choose between a borrowed literal and an owned `String` up front.
