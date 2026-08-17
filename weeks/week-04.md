# Week 4 — Error Handling

**Goal:** Master `Result<T, E>` — Rust's approach to errors as *values* rather than exceptions. This is a big mindset shift from Java/Python/TS and it's what makes Rust code so reliable.

**Facet:** paradigm.

---

## Map from your languages

| Rust | Java | Python | TypeScript |
|------|------|--------|-----------|
| `Result<T,E>` | checked exceptions (rare) | exceptions | thrown errors |
| `?` operator | — | — | — |
| `unwrap()` / `expect()` | — | — | — |
| `panic!` | unchecked exception | exception | uncaught error |
| custom error types | custom exception classes | custom exceptions | `Error` subclasses |

**The key difference:** in Java/Python/TS, errors are *thrown* and fly up the call stack invisibly until caught. In Rust, a function that can fail **returns** a `Result` — the failure is part of the function's type signature. The compiler forces you to handle it (or explicitly defer it with `?`). You can't forget to handle an error.

---

## Day 1 (Mon) — `Result` basics and `?`

**Recall (2 min):** What's the difference between `Option<T>` and `Result<T, E>`? (Option: maybe a value. Result: a value *or an error*.)

**Theory (10 min):**
- `Result<T, E>` is `Ok(T)` or `Err(E)`.
- `unwrap()` — take the `T` or panic. `expect("msg")` — same but with a message. **Use these while learning; replace them later.**
- The **`?` operator** is the real tool: inside a function returning `Result`, `x?` unwraps `Ok` or *returns early* with `Err`. It's like "throw" but explicit and type-checked.
- `main` can return `Result`: `fn main() -> Result<(), Box<dyn std::error::Error>>` — lets you use `?` in main.

**Exercise (15 min):**
```rust
use std::num::ParseIntError;

fn parse_num(s: &str) -> Result<i32, ParseIntError> {
    s.trim().parse::<i32>()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a = parse_num("42")?;
    let b = parse_num("17")?;
    println!("{} + {} = {}", a, b, a + b);
    Ok(())
}
```
Then: call `parse_num("not a number")` and run it — watch the program exit with an error message (that's `?` propagating to main). Then write a version that catches the error with `match` and prints a friendly message instead of crashing.

**Reflect (3 min):** Why is it better that `parse_num` *returns* an error instead of throwing one? (The caller can't silently ignore it — the type system says "this can fail.")

---

## Day 2 (Tue) — Custom error types

**Recall (2 min):** What does `?` do? What must a function return to use it?

**Theory (10 min):** In real code you define your own error type — an enum of everything that can go wrong:
```rust
use std::fmt;

#[derive(Debug)]
enum ConfigError {
    MissingField(&'static str),
    InvalidNumber(String),
    Io(std::io::Error),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::MissingField(name) => write!(f, "missing field: {name}"),
            ConfigError::InvalidNumber(v) => write!(f, "invalid number: '{v}'"),
            ConfigError::Io(e) => write!(f, "io error: {e}"),
        }
    }
}

impl std::error::Error for ConfigError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ConfigError::Io(e) => Some(e),
            _ => None,
        }
    }
}
```
- `Display` makes it printable; `std::error::Error` makes it a first-class error.
- `?` uses `From` to convert: if your error has `Io(std::io::Error)`, then `?` on an `io::Result` converts automatically **if** you implement `From<std::io::Error>` (or use the `thiserror` crate, which generates all this).

**Exercise (15 min):** Parse a mini config string into a `Config` struct:
```
host=localhost
port=8080
```
Write `parse_config(&str) -> Result<Config, ConfigError>` using the enum above. Handle: missing `host`, `port` not a number, and a file-read error (`std::fs::read_to_string` → convert with `?`). Test it with good and bad inputs. If you're comfortable, add `thiserror` (`cargo add thiserror`) and re-derive the same error with `#[derive(thiserror::Error)]` — see how much boilerplate it saves.

**Reflect (3 min):** Why is an *enum* the right shape for an error type? (It enumerates every failure mode — documentation and matchable.)

---

## Day 3 (Wed) — Option & Result combinators

**Recall (2 min):** What two traits must a custom error type implement?

**Theory (10 min):** Combinators transform `Option`/`Result` without `match`:
- On `Option`: `map`, `and_then`, `unwrap_or`, `unwrap_or_else`, `ok_or`, `filter`, `is_some`, `is_none`
- On `Result`: `map`, `map_err`, `and_then`, `unwrap_or`, `is_ok`, `is_err`
- `ok_or(E)` converts `Option` → `Result` (attach an error if `None`).

**Compare:**
```rust
// Nested match (verbose)
let n = match opt {
    Some(s) => match s.parse::<i32>() {
        Ok(v) => v,
        Err(_) => 0,
    },
    None => 0,
};

// Combinators (concise)
let n = opt.and_then(|s| s.parse().ok()).unwrap_or(0);
```

**Exercise (15 min):** Write a function that looks up a user's age from a `HashMap<String, u8>`, given a name, and returns `Result<u8, String>` ("user not found"). Implement it first with `match`, then with combinators (`ok_or`, `and_then`). Then chain: parse a number from an env var, defaulting to 8080, using `.ok().unwrap_or(8080)`.

**Reflect (3 min):** When is `match` clearer than combinators? (When there are many cases or you need `Err` details. Combinators shine for simple "map or default" chains.)

---

## Day 4 (Thu) — `panic!` and error-handling philosophy

**Recall (2 min):** What does `ok_or` do?

**Theory (10 min):** Two kinds of failure:
- **`panic!`** — unrecoverable, programmer errors, broken invariants. The program aborts (or unwinds). Like an unchecked exception you *meant* to crash on.
- **`Result`** — recoverable, expected failures (bad input, missing file, network). You *must* handle these.

**Rules of thumb:**
- Panic when: an invariant is broken (`x / y` where `y == 0`), unreachable states, tests.
- Return `Result` when: the failure is a *possible, expected* outcome.
- `unwrap()`/`expect()` in example code and prototypes are fine; in production, replace with `?` or handling.
- `expect` > `unwrap`: the message documents *why* it can't fail: `.expect("we checked the length first")`.

**Exercise (15 min):** Write a function `fn divide(a: f64, b: f64) -> Result<f64, String>` that returns `Err("division by zero")` for `b == 0.0`. Then write a version that panics instead. Then write a small program where you *deliberately* cause a panic (e.g., `let v = vec![1]; v[5];`) and read the panic message — note it tells you the file and line. Finally, refactor any `.unwrap()` you wrote this week into proper `?`/handling.

**Reflect (3 min):** For your Ticket model from Week 3, should validation errors be panics or `Result`? (Result — bad input is expected, not a bug.)

---

## Day 5 (Fri) — Practice: a file-processing CLI

**Recall (2 min):** When should you panic vs return `Result`?

**Exercise (25 min):** Build a `word_count` CLI:
1. Read a filename from `std::env::args()`.
2. `std::fs::read_to_string` the file (handle the missing-file case with `?` and a custom error).
3. Count words (split on whitespace), lines, and characters.
4. Print a nice report.
5. Handle errors: no filename given, file doesn't exist, file unreadable — each with a clear message and a non-zero exit code.

Structure it with a custom `CliError` enum (or `thiserror`), a `run() -> Result<(), CliError>` function, and a thin `main` that matches on the result. Run `clippy --pedantic` and make it clean.

**Reflect (3 min):** Compare this to how you'd do it in Python (try/except). What did Rust's type system force you to think about that Python wouldn't?

---

## Weekend checkpoint (~15 min)

**Take the quiz:** [Week 4 quiz](../quizzes/week-04.md) — closed-book, then check the answers. Mark your misses in the quiz file.

1. From memory: (a) `?` — what does it do, and what must the function return? (b) the two traits for custom errors; (c) three combinators and what they do; (d) panic vs Result.
2. Redo the `word_count` CLI from scratch — no looking. Time yourself; it should be faster than Friday.
3. Update `progress.md` (error log: add any >5-min errors).

## Reading assignment (10 min)

The Book **chapter 9** (errors). Then look at how a real crate structures errors — open `anyhow` or `serde_json` on docs.rs and find its error type. Notice: real crates have a small, well-named error enum, and `?` does the heavy lifting.

## Resources

- The Book, ch. 9
- Rustlings: `errors`, `result`
- 100 Exercises to Learn Rust: exercise 6+ (error-handling ticket)
