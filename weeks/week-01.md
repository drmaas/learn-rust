# Week 1 — Syntax, Types, Control Flow, and Cargo

**Goal:** Get comfortable reading and running Rust, and build the "Rust mental model": values live somewhere, and everything is explicit. This week is mostly *easy* because you know Java/Python/TS — the point is to get the tooling and rhythm working.

**Facets covered:** syntax & semantics, paradigm (expression-oriented), ecosystem (Cargo), tooling (`fmt`, `clippy`, rust-analyzer).

---

## Map from your languages

| Rust | Java | Python | TypeScript |
|------|------|--------|-----------|
| `fn main()` | `public static void main` | `if __name__ == "__main__"` | entry point |
| `let x = 5;` (immutable) | `final int x = 5;` | `x = 5` | `const x = 5` |
| `let mut x = 5;` | `int x = 5;` | `x = 5` | `let x = 5` |
| `println!("{}", x)` | `System.out.println` | `print(x)` | `console.log(x)` |
| `i32`, `u64`, `f64`, `bool` | `int`, `long`, `double`, `boolean` | int, float, bool | `number`, `boolean` |
| `String` / `&str` | `String` | `str` | `string` |
| `match` | `switch` | `match` | `switch` |
| `if` is an *expression* | ternary only | — | ternary |
| `for x in iter` | enhanced for | `for x in iter` | `for..of` |

**The one big difference this week:** in Rust, `if`/`match`/blocks are **expressions** — they produce a value. There's no ternary operator because you don't need one.

---

## Day 1 (Mon) — Setup, hello world, Cargo

**Recall (2 min):** First day — instead of a quiz, write down 3 things you expect to be *different* in Rust compared to Java/Python, and 3 things you expect to be *the same*. Keep this list; check it at the end of the week.

**Theory (10 min):** What Cargo is. If you've used Maven/Gradle/npm, this is familiar:
- `cargo new NAME` — creates a project: `Cargo.toml` (manifest, like `package.json`/`pom.xml`) and `src/main.rs`.
- `cargo run` — build + run. `cargo build` — build only. `cargo test` — tests. `cargo fmt` — format. `cargo clippy` — lint.
- `Cargo.toml` holds dependencies; `cargo add serde` adds one (like `npm install`).

**Exercise (15 min):**
```bash
cargo new hello
cd hello
```
Open `src/main.rs`, change the message, and:
```bash
cargo run
cargo fmt        # watch it reformat your code
cargo clippy     # should be clean
cargo test       # runs a default test
```
Then **break it on purpose**: delete a semicolon, run `cargo run`, and read the error message carefully. Rust's compiler is famous for helpful errors — get used to reading them now.

**Reflect (3 min):** This session touched all four facets (syntax, paradigm, ecosystem, tooling). In `progress.md`, write which facet each command belongs to.

---

## Day 2 (Tue) — Variables, mutability, shadowing, primitives

**Recall (2 min):** What does `cargo run` do? What's in a new project? *Write it down before looking.*

**Theory (10 min):**
- `let` binds are **immutable by default**. `let mut` makes them mutable. Rust makes you say what you mean.
- **Shadowing** ≠ mutation: `let x = 5; let x = x + 1;` creates a *new* binding that hides the old one. This is a feature (used for transformations).
- Primitives: integers `i8..i128` / `u8..u128` (signed/unsigned), floats `f32`/`f64`, `bool`, `char` (4 bytes, Unicode).
- Type inference: `let x = 5;` infers `i32`. You can annotate: `let x: u64 = 5;`.

**Exercise (15 min):** Write a program that converts Fahrenheit to Celsius. Predict the output of this *before* running it:

```rust
fn main() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("inner: {x}");
    }
    println!("outer: {x}");
}
```
Then write the temperature converter with explicit types (`let f: f64 = 98.6;`), and try integer vs float division (`5 / 2` vs `5.0 / 2.0`) — note the difference from Python.

**Reflect (3 min):** In your own words: what's the difference between shadowing and mutation? (This trips up most people coming from Python.)

---

## Day 3 (Wed) — Control flow, expressions, functions

**Recall (2 min):** Difference between shadowing and mutation? What type does `let x = 5;` infer?

**Theory (10 min):**
- **Blocks are expressions.** The last line of a block is its value. So `if` can produce a value:
  ```rust
  let grade = if score >= 90 { "A" } else { "B" };
  ```
  No ternary, no `? :`, no `return` needed. Braces `{}` are the universal expression wrapper.
- `loop` (infinite), `while`, `for x in 0..10` (ranges are iterators), `break`/`continue`.
- Functions: `fn name(a: i32, b: i32) -> i32 { a + b }` — the last expression is the return value; `return` is only for early exits.
- `match` (preview — full treatment in Week 3): like a powerful `switch` that *must* cover all cases.

**Exercise (15 min):** Port FizzBuzz from your favorite language to Rust, but with a twist: write it using an `if` expression that assigns a `String`, and also write a version using `match` on `(n % 3, n % 5)`. Run `clippy` and fix anything it flags.

```rust
fn fizzbuzz(n: u32) -> String {
    match (n % 3, n % 5) {
        (0, 0) => "FizzBuzz".to_string(),
        (0, _) => "Fizz".to_string(),
        (_, 0) => "Buzz".to_string(),
        _ => n.to_string(),
    }
}
```
Predict: what does `fizzbuzz(15)` return? `fizzbuzz(7)`? Then run it.

**Reflect (3 min):** Why does Rust use expressions instead of a ternary? (Hint: it's the same reason `match` is exhaustive — no silent fall-through bugs.)

---

## Day 4 (Thu) — Strings and slices

**Recall (2 min):** What does "if is an expression" mean? Write a one-liner that uses `if` to assign a value.

**Theory (10 min):**
- Rust has **two** string types. This is the first real "Rust is different" moment:
  - `String` — owned, growable, heap-allocated. Like Java's `String` (but mutable if you want).
  - `&str` — a *borrowed* string slice; a view into a `String` (or into the binary). Like a read-only reference.
- `"hello"` in your code is a `&'static str` (a literal baked into the binary).
- Converting: `"hello".to_string()` or `String::from("hello")`; `&my_string[..]` or `my_string.as_str()` to borrow.
- `format!("{} {}", a, b)` — like Python's f-strings / JS template literals.
- `+` on `String` takes a `&str` on the right: `let s = String::from("a") + "b";`.
- `char` is a single Unicode scalar; `&str` is UTF-8 bytes. `s.len()` is *bytes*, not characters — `s.chars().count()` is characters. (Python's `len` is characters — this difference bites people.)

**Exercise (15 min):** Write a program that: takes a name from `std::env::args()` (or hardcode it), builds a greeting with `format!`, prints its byte length and character count, and slices the first 3 characters. Try `s[0]` on a `String` — read the error (you can't index a `String` by integer; the error tells you why).

**Reflect (3 min):** Why does `"hello".to_string()` exist — why isn't `"hello"` already a `String`? (Answer: literals live in the binary, not the heap.)

---

## Day 5 (Fri) — Practice: port a program you already know

**Recall (2 min):** `String` vs `&str` — which is owned? Which is a view?

**Exercise (25 min):** Pick a small program you've written in Java or Python and port it to Rust:
- a simple calculator (read two numbers and an operator, print the result), or
- a number-guessing game (`rand` crate optional — or use a fixed target), or
- a text-based score classifier.

**Rules:** type it by hand, no autocomplete. Compile, read every error, fix, run `clippy --pedantic` (`cargo clippy -- -W clippy::pedantic`) and fix what it flags. If you finish early, do the first 3 **Rustlings** exercises (`cargo install rustlings`, `rustlings init`, `rustlings watch`).

**Reflect (3 min):** What was the hardest part? Was it the syntax, or a different mental model? Log it.

---

## Weekend checkpoint (~15 min)

**Take the quiz:** [Week 1 quiz](../quizzes/week-01.md) — closed-book, then check the answers. Mark your misses in the quiz file.

1. Answer from memory: (a) what are the four facets of a language? (b) shadowing vs mutation? (c) `String` vs `&str`? (d) how do you make a `String` from a literal?
2. Redo the FizzBuzz exercise from scratch, from memory. No looking.
3. Check the "3 differences / 3 similarities" list you wrote on Day 1. Were you right?
4. Update `progress.md`: fill in your concept map rows for this week.

## Reading assignment (optional, 10 min)

Skim one real Rust file — e.g., the docs/source of `std::env::args` (open `https://doc.rust-lang.org/std/env/fn.args.html` and click `[src]` to see the source). You won't understand most of it. That's fine — you're building pattern recognition. Note anything that looks familiar.

## Resources

- The Book, ch. 1–3 (getting started, guessing game, common concepts)
- Rust by Example: 1–5
- Rustlings: `variables`, `functions`, `if`
