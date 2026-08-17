# Week 7 Quiz — Modules, Crates, Testing, Docs

**Missed questions (fill in as you go):**

---

## Questions (answer closed-book, in writing)

1. Module vs crate — what's the difference?

2. What does `pub` do? What do `pub(crate)` and `pub(super)` mean?

3. Where do **integration tests** live, and what do they test that unit tests don't?

4. What is a **doc test**? How does `cargo test` use your documentation?

5. What does `cargo add serde --features derive` do?

6. What is a **workspace**? Why would a project like Tauri be a workspace?

7. `#[test]` vs `#[should_panic]` — what does each do?

8. What does `#![warn(missing_docs)]` do?

9. `cargo doc` vs `cargo build` — what does each produce?

10. Unit tests vs integration tests — which one can access a crate's *private* items?

11. **Write code:** a unit test module for a function `fn double(x: i32) -> i32 { x * 2 }` (with `assert_eq!`).

12. **Map from your languages:** Maven/Gradle (Java), pip (Python), npm (JS) — which Rust tool is the equivalent of all three, and what are its three main commands?

---

## Answers & Explanations

1. A **crate** is a compilation unit — a library (`lib.rs`) or binary (`main.rs`) that gets compiled into one artifact. **Modules** organize code *within* a crate (a tree of namespaces). A crate can contain many modules; a workspace can contain many crates.

2. `pub` makes an item visible **outside its module** (and outside the crate if it's on the public path). `pub(crate)` limits visibility to the crate. `pub(super)` limits it to the parent module. **Privacy is the default** — everything starts private.

3. Integration tests live in the **`tests/` directory** (one file per test suite). They test the crate's **public API exactly as an external consumer would** — they can only use `pub` items. Unit tests live inside modules (`#[cfg(test)] mod tests`) and can access private internals. Both are valuable: unit tests for internals, integration tests for the contract.

4. A doc test is a **code block inside a `///` doc comment** that `cargo test` compiles and runs. If your documentation example doesn't compile or produces wrong output, the test **fails**. This means your docs can never rot — they're always verified. (This is a Rust superpower Java's Javadoc and Python's docstrings don't have.)

5. It adds `serde` as a dependency **with the `derive` feature enabled**, which lets you use `#[derive(Serialize, Deserialize)]`. Features are optional capabilities of a crate; you enable only what you need (Tauri and tokio have many).

6. A **workspace** is a set of related crates in one repo, sharing a single `Cargo.toml` (`[workspace]`), a lockfile, and a build. Projects like Tauri use workspaces because they're built from many crates (`tauri`, `tauri-runtime`, `tauri-macros`, etc.) that depend on each other — one repo, one test run, coherent versioning.

7. `#[test]` marks a function as a test — run by `cargo test`. `#[should_panic]` marks a test that is *expected to panic* — the test **passes if it panics** and fails if it doesn't. Use it for testing panic paths (e.g., an assertion on a broken invariant).

8. `#![warn(missing_docs)]` (at the top of a crate) makes the compiler warn about every **public item without a doc comment**. It's a nag that forces you to document your public API — a quality bar real crates use. `#![deny(missing_docs)]` turns it into a hard error.

9. `cargo build` produces a **compiled binary/library**. `cargo doc` produces **HTML documentation** (from `///` comments) in `target/doc/` — `cargo doc --open` opens it in the browser. It's like `javadoc`, but with runnable examples.

10. **Unit tests** can access private items (they're compiled inside the crate with `#[cfg(test)]`). **Integration tests** are separate crates that can only use the public API. This is by design: unit tests test implementation details; integration tests test the contract.

11.
   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;

       #[test]
       fn doubles_positive() {
           assert_eq!(double(3), 6);
       }

       #[test]
       fn doubles_negative() {
           assert_eq!(double(-4), -8);
       }
   }
   ```
   (`#[cfg(test)]` means this module compiles only when running tests.)

12. **Cargo** is the combined package manager + build tool + test runner. The three main commands: `cargo build` (compile), `cargo test` (run tests), `cargo add` (add a dependency, like `pip install`/`npm install`). Plus `cargo run`, `cargo fmt`, `cargo clippy`, `cargo doc`.
