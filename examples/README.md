# Verified Course Examples

Every code example from the **Learn Rust in 12 Weeks** course, implemented as
real, compilable Rust with `#[test]`s that **assert the documented behavior**.

Run everything with:

```bash
cd learn-rust/examples
cargo test     # 102 tests, all green
cargo clippy   # clean (0 warnings)
```

Expect all tests green. That's the point — the course's examples are not
"trust me, this works"; they're verified.

**Quality bar:** default `clippy` is clean, and `clippy::pedantic` is enforced
on the `capstone` module. The other week modules intentionally preserve some
teaching-oriented snippets, so pedantic is not enforced across the entire
examples crate. The capstone is the production-style boundary: complete API
documentation, `#[must_use]` annotations, and no pedantic warnings.

---

## How it maps to the course

| Module | Course material |
|--------|-----------------|
| `learn` | HOW-YOU-LEARN.md (ownership + lifetime examples) |
| `w1` … `w12` | One module per course week |
| `capstone` | The Week 12 ClipNote core (Note, Store, search, commands) |
| `tests/plugin_registry.rs` | Week 6/7 plugin system, used from outside (integration test) |
| `tests/capstone_store.rs` | Capstone store, used from outside |

## How the example types are handled

- **Working examples** → implemented as functions, asserted with `#[test]`.
- **"Predict the output" examples** → the expected output is the assertion
  (e.g. the Week 1 shadowing example asserts `["inner: 12", "outer: 6"]`).
- **"Does this compile?" / deliberately-broken examples** → shown as comments
  with the expected compiler error, and the **corrected** version is tested.
- **Runtime panics the course asks you to trigger** (e.g. the `RefCell`
  double-borrow in Week 8) → real `#[should_panic]` tests. The panic is the
  lesson, so the test asserts it happens.
- **Tauri `#[tauri::command]` examples (Weeks 11–12)** → the command *logic*
  is implemented as plain functions and tested. The `#[tauri::command]`
  attribute itself needs the full `tauri` crate (which requires system webview
  libraries to build), so it isn't a dependency here — it's framework plumbing
  verified by Tauri's own test suite.
- **Examples that crash the whole process** (e.g. dereferencing a null pointer
  in Week 10) → kept as documented exercises, not run as tests.

## Dependencies

`tokio`, `futures`, `serde`/`serde_json`, `thiserror`, `libc` (all used by the
course examples), plus `tempfile` as a dev-dependency for file-based tests.

## Adding a new example

1. Add the function to the relevant `src/wN.rs` module.
2. Add a `#[test]` asserting its documented behavior.
3. Run `cargo test` — it must pass.
4. If adding code to `src/capstone.rs`, run `cargo clippy --all-targets` and
   keep the module pedantic-clean.
