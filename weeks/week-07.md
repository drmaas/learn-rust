# Week 7 — Modules, Crates, Testing, Documentation

**Goal:** Learn how real Rust projects are *organized* — this is the "contribution skills" week. Every open-source Rust project you'll ever read is structured with modules, crates, tests, and docs.

**Facet:** ecosystem & tooling.

---

## Map from your languages

| Rust | Java | Python | TypeScript |
|------|------|--------|-----------|
| `mod` / `use` | packages | modules/imports | modules |
| crate | JAR/module | package | package |
| `cargo add` | Maven/Gradle | pip | npm install |
| `#[test]` | JUnit | pytest | Jest |
| `///` docs + rustdoc | Javadoc | docstrings | JSDoc |

---

## Day 1 (Mon) — Modules and visibility

**Recall (2 min):** Why is `Box<dyn Plugin>` the right type for a plugin registry?

**Theory (10 min):**
- A crate is a compilation unit (a library `lib.rs` or binary `main.rs`). Modules organize code *within* a crate.
- `mod foo;` declares a module (looks for `foo.rs` or `foo/mod.rs`). `mod foo { ... }` inlines one.
- **Visibility is private by default.** `pub` makes it public; `pub(crate)` = visible in the crate only; `pub(super)` = parent module.
- `use crate::foo::Bar;` brings names into scope. `pub use` re-exports (so consumers see a clean API).
- The module tree mirrors the file tree; `crate` is the root.

**Exercise (15 min):** Convert your Week 6 plugin system into a **library crate**:
1. `cargo new --lib my_plugins`
2. `src/lib.rs` with `pub mod plugins;` and `pub mod registry;`
3. `src/plugins.rs` defines the `Plugin` trait + the 3 implementations (mark what's `pub`).
4. `src/registry.rs` defines `Registry`.
5. Add a `src/bin/demo.rs` (a binary that uses the library) — this shows the lib + bin layout.
6. Try to use a private item from another module and read the "private" error.

**Reflect (3 min):** Why is privacy the default in Rust but not in Python? (Privacy is a contract — the compiler enforces what you can change without breaking consumers. Python relies on convention: `_name`.)

---

## Day 2 (Tue) — Crates and dependencies

**Recall (2 min):** What does `pub(crate)` mean?

**Theory (10 min):**
- `Cargo.toml` `[dependencies]` section; `cargo add serde` adds the latest version.
- **Semver:** `serde = "1.0"` means `>=1.0.0, <2.0.0` — compatible updates only.
- `[dev-dependencies]` — only for tests/examples (e.g., `criterion` for benchmarks).
- `[features]` — optional capabilities (like `--features` flags); used heavily by Tauri and tokio.
- `cargo tree` shows the dependency graph; `cargo update` bumps versions.
- **Workspaces:** multiple related crates in one repo sharing a lockfile (`[workspace]` in root `Cargo.toml`). Tauri is a workspace of ~10 crates — you'll see this in Week 11.

**Exercise (15 min):** In your `my_plugins` crate, `cargo add anyhow` and `cargo add serde --features derive`. Use `anyhow::Result` in a new function (this is the pragmatic error type for apps). Add a `#[derive(Serialize, Deserialize)]` struct and serialize it to JSON with `serde_json` (`cargo add serde_json`). Run `cargo tree` and look at the graph.

**Reflect (3 min):** What's the difference between a dependency and a dev-dependency? (Dev-deps don't ship to consumers; tests/benches only.)

---

## Day 3 (Wed) — Testing

**Recall (2 min):** What does `cargo add serde --features derive` do?

**Theory (10 min):**
- Unit tests live next to code in `#[cfg(test)] mod tests { ... }` — compiled only when testing.
- `#[test]` marks a test; `assert!`, `assert_eq!`, `assert_ne!`; `#[should_panic]` for panic tests.
- **Integration tests** live in `tests/` and test the crate's *public API* from outside (like a consumer would).
- **Doc tests**: code blocks in `///` docs are compiled and run by `cargo test` — your docs are guaranteed correct.
- `cargo test`, `cargo test <name>` (filter), `cargo test -- --nocapture` (show prints).
- `#[ignore]` + `cargo test -- --ignored` for slow tests.

**Exercise (15 min):** Write thorough tests for your plugin system:
1. Unit tests in each module (`#[cfg(test)] mod tests`) covering every plugin + registry method.
2. Integration tests in `tests/registry.rs` that use the public API.
3. Add a doc example to the `Plugin` trait's doc comment and verify `cargo test` runs it.
4. Add a `#[should_panic]` test — e.g., a debug-only assertion.

**Reflect (3 min):** What's the difference between a unit and an integration test? (Unit: internal, can see private. Integration: public API only, from `tests/`.) Why have both?

---

## Day 4 (Thu) — Documentation

**Recall (2 min):** Where do integration tests live, and what do they test?

**Theory (10 min):**
- `///` doc comments (Markdown) → `cargo doc` → rustdoc HTML (like Javadoc, but with live examples).
- `//!` is a module-level doc comment (at the top of `lib.rs`, becomes the crate docs).
- Doc examples run as tests (see Day 3).
- Structure of good docs: one-line summary, `# Examples`, `# Errors`, `# Panics`, `# Safety` (for unsafe).
- `cargo doc --open` builds and opens. `#![warn(missing_docs)]` makes the compiler nag you until everything's documented.
- A good README + docs + examples is what makes a crate *usable* — and what maintainers look for in contributors.

**Exercise (15 min):** Document your `my_plugins` crate:
1. Add `//!` crate-level docs to `lib.rs`.
2. Document the `Plugin` trait, each method, and `Registry` with `///` + a working example.
3. Add `#![warn(missing_docs)]` and fix every warning.
4. Run `cargo doc --open` and browse your generated docs. Add a README with a short usage example.

**Reflect (3 min):** Why do Rust projects treat docs as tests? (Doc examples that compile = docs that never lie. Javadoc can rot; rustdoc examples can't.)

---

## Day 5 (Fri) — Practice: a small standalone library crate

**Recall (2 min):** What does `#![warn(missing_docs)]` do?

**Exercise (25 min):** Build a small but *complete* library crate from scratch — something useful and testable, e.g.:
- a **unit converter** (temperature, length, weight) with a `ConvertError` for unknown units, or
- a **text formatter** (markdown-lite: bold, headers, lists) or
- a **word-frequency** library that returns a sorted `Vec<(String, u32)>`.

Requirements (the "publishable crate" checklist):
1. Library crate with 2+ modules, clean public API (`pub use` re-exports).
2. `thiserror`-based error type.
3. Unit + integration tests, all passing.
4. Full docs with runnable examples, `missing_docs` clean.
5. `clippy --pedantic` clean.
6. A README with usage.

**Reflect (3 min):** Would you be comfortable publishing this to crates.io? What's missing? (This is the bar real maintainers expect.)

---

## Weekend checkpoint (~15 min)

**Take the quiz:** [Week 7 quiz](../quizzes/week-07.md) — closed-book, then check the answers. Mark your misses in the quiz file.

1. From memory: (a) module vs crate; (b) unit vs integration test; (c) how doc tests work; (d) what `cargo tree` shows.
2. Add one new feature to your library crate and re-run the whole checklist (tests, docs, clippy).
3. Update `progress.md`.

## Reading assignment (10 min)

The Book **chapters 7, 11, 14** + skim the Cargo book (`https://doc.rust-lang.org/cargo/`). Then open a small real crate on crates.io (e.g., `anyhow` or `thiserror`) and browse its docs.rs page: note the module structure, the `//!` crate docs, and the doc examples. This is what your crate should look like.

## Resources

- The Book, ch. 7, 11, 14
- Cargo book: https://doc.rust-lang.org/cargo/
- Rustlings: `modules`, `tests`
