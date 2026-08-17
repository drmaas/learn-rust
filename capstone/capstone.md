# Capstone — "ClipNote": a Tauri notes manager with a plugin

**When:** Week 12 (extend into Week 13 if needed — it's your project now).
**Goal:** A working desktop app that proves you can build and read real Rust: data model, error handling, iterators, traits/plugins, concurrency, async, tests, docs, and navigating Tauri's API.

The app: **ClipNote** — a simple local notes manager. Create, edit, delete, and search notes stored as files on disk, with a stats plugin that analyzes your notes. It's deliberately small on features so the *Rust* is the point.

---

## What each milestone exercises (map to your weeks)

| Milestone | Rust skills used | Learned in |
|-----------|------------------|------------|
| M0: Scaffold | Tauri setup, tooling | Week 11 |
| M1: Core library | structs/enums, `Result`, custom errors, `Vec`/`HashMap`, iterators, tests | Weeks 3–7 |
| M2: Commands + State + IPC | `#[tauri::command]`, `State`, `Mutex`, async commands, error propagation | Weeks 4, 9, 11 |
| M3: Plugin | traits, trait objects, plugin architecture | Weeks 6, 11 |
| M4: Polish | docs, `clippy --pedantic`, README, code review | Week 7 |

---

## The design

```
clipnote/                          (a cargo workspace)
├── Cargo.toml                     (workspace)
├── apps/
│   └── clipnote-app/              (the Tauri application)
│       ├── src/
│       │   ├── main.rs            (tauri::Builder wiring)
│       │   ├── commands.rs        (all #[tauri::command] fns)
│       │   └── state.rs           (AppState)
│       └── ui/                    (frontend: index.html + app.js)
└── crates/
    ├── clipnote-core/             (pure library: no Tauri!)
    │   └── src/
    │       ├── lib.rs
    │       ├── note.rs            (Note model + validation)
    │       ├── store.rs           (file-backed storage)
    │       ├── search.rs          (search logic)
    │       └── error.rs           (ClipError)
    └── tauri-plugin-clipnote-stats/  (the plugin, as its own crate)
```

**Why split it this way:** the core library has *no Tauri dependency* — it's pure Rust you can test with plain `cargo test`. This is exactly how real projects (and Tauri itself) separate logic from framework. It also means your core is reusable and portable.

---

## Milestone 0 — Scaffold + design (~1 session)

1. Install Tauri Linux deps (SETUP.md) if not done.
2. `sh <(curl https://create.tauri.app/sh)` — choose **vanilla** (plain HTML/JS) to keep the frontend simple, Rust as the language.
3. `cargo tauri dev` — confirm the default window opens.
4. Set up the workspace: put the app in `apps/clipnote-app/`, create `crates/clipnote-core` and the plugin crate.
5. **Write the design down** in `progress.md` before coding: the module layout, the command list, the error types.

**Definition of done:** default app runs; workspace compiles; you have a written design.

---

## Milestone 1 — Core library (1–2 sessions)

All pure Rust, no Tauri. In `crates/clipnote-core`:

**`note.rs`** — the model:
```rust
pub struct Note {
    pub id: u64,
    pub title: String,
    pub body: String,
    pub created_at: std::time::SystemTime,
}
```
- `Note::new(title, body)` — **validate**: title non-empty and ≤ 120 chars; body ≤ 10_000 chars. Return `Result<Note, ClipError>` (reuse your Week 3/4 skills: enum + `thiserror`).

**`store.rs`** — file-backed storage:
- `Store::new(dir: PathBuf)` — creates the directory if missing.
- `save(&self, note: &Note) -> Result<(), ClipError>` — write one file per note (`{id}.json`).
- `load_all(&self) -> Result<Vec<Note>, ClipError>` — read all notes, sorted by `created_at`.
- `get(&self, id: u64) -> Result<Option<Note>, ClipError>`.
- `delete(&self, id: u64) -> Result<(), ClipError>`.
- Use `serde` (`#[derive(Serialize, Deserialize)]`) for JSON. Handle **every** error with `?` into `ClipError`.

**`search.rs`** — pure functions:
- `search(notes: &[Note], query: &str) -> Vec<&Note>` — case-insensitive substring match on title + body, using iterators (`filter`).
- `word_frequency(notes: &[Note]) -> Vec<(String, u32)>` — most common words across all bodies (uses `HashMap` + `entry`), sorted descending (Week 5 skills).

**`error.rs`** — `ClipError` enum: `Io(std::io::Error)`, `InvalidNote(String)`, `Json(serde_json::Error)`, `NotFound(u64)`. `#[derive(thiserror::Error)]` + sensible messages.

**Tests (this is mandatory):** unit tests for validation, store round-trip (save → load → delete, using a temp dir), search, and word frequency. Use `tempfile` (`cargo add --dev tempfile`) for test dirs.

**Definition of done:** `cargo test` green; `clippy --pedantic` clean; the core has zero Tauri imports.

---

## Milestone 2 — Commands + State + IPC (1 session)

In `apps/clipnote-app`:

**`state.rs`:**
```rust
pub struct AppState {
    pub store: std::sync::Mutex<clipnote_core::store::Store>,
}
```

**`commands.rs`** — sync and async commands:
- `create_note(state, title, body) -> Result<u64, String>` — sync; validates via `Note::new`.
- `list_notes(state) -> Result<Vec<Note>, String>` — sync.
- `get_note(state, id) -> Result<Note, String>` — sync.
- `delete_note(state, id) -> Result<(), String>` — sync.
- `search_notes(state, query) -> Result<Vec<Note>, String>` — **async** (simulate a brief delay with `tokio::time::sleep`) — exercises async commands.
- `note_stats(state) -> Result<Vec<(String, u32)>, String>` — calls `word_frequency`; make this **async** too.

**Error mapping:** convert `ClipError` → `String` with friendly messages (e.g., `NotFound` → "Note not found"). The frontend should never see a panic — only clean errors.

**`main.rs`:** `.manage(AppState { ... })`, `.invoke_handler(tauri::generate_handler![...])`, `.plugin(...)` (added in M3).

**Frontend (`index.html` + `app.js`):** a textarea + title input + buttons for create/list/delete/search, and a "stats" button. Call with `window.__TAURI__.core.invoke('create_note', { title, body })` (v2 API). Show errors in red text on the page.

**Definition of done:** you can create, list, delete, search, and see stats in the running app — with errors displayed nicely.

---

## Milestone 3 — The plugin (1 session)

Extract stats into a real plugin crate, `crates/tauri-plugin-clipnote-stats`:

1. Follow the [Tauri plugin guide](https://v2.tauri.app/develop/plugins/).
2. The plugin exposes a command `stats(store: State<...>) -> Result<Vec<(String, u32)>, String>` that computes word frequency (it can depend on `clipnote-core` for the logic).
3. Give the plugin a `setup` hook that logs "clipnote-stats plugin initialized" (see `tauri::plugin::Builder::setup`).
4. Register it in the app: `.plugin(tauri_plugin_clipnote_stats::init())` and remove the in-app `note_stats` command.
5. Add a **capability/permission** entry in `capabilities/` declaring the command the frontend may call (Tauri v2 requires this).

**Definition of done:** the app still shows stats, but now through the plugin; removing the plugin crate breaks the build (you can verify by commenting it out).

---

## Milestone 4 — Polish + review (1–2 sessions)

1. **Clippy:** `cargo clippy --workspace --all-targets -- -W clippy::pedantic` — fix everything.
2. **Docs:** `//!` + `///` on the core crate's public items; `#![warn(missing_docs)]` clean; `cargo doc --open` looks professional.
3. **README** for the whole project: what it is, how to run, screenshots optional.
4. **Tests:** run the full suite; add any missing edge-case tests (e.g., delete a nonexistent note → `NotFound`).
5. **Error UX:** make sure every failure path shows a friendly message in the UI (test by: deleting a note twice, searching with an empty query, creating a note with an empty title).
6. **Manual test pass:** run the app, do every action, note anything that feels broken.

**Definition of done:** the rubric below is fully checked.

---

## The rubric (check everything)

**Core Rust**
- [ ] `Note` validation returns `Result`, not panics
- [ ] Custom `ClipError` enum with `thiserror`; `?` used throughout
- [ ] Ownership/borrowing: no unnecessary `.clone()`s in hot paths; state behind `Mutex`
- [ ] Iterators used for search and frequency (no manual index loops)
- [ ] Async command(s) present and correct
- [ ] `unsafe` used: **zero** (you don't need it here — and that's a good outcome)

**Architecture**
- [ ] Core library has no Tauri dependency and is fully unit-tested
- [ ] Plugin is a separate crate registered via `.plugin(...)`
- [ ] Capabilities/permissions configured for the plugin command
- [ ] Workspace layout clean (apps/ + crates/)

**Quality**
- [ ] `cargo test --workspace` green
- [ ] `cargo clippy --workspace --all-targets -- -W clippy::pedantic` clean
- [ ] `#![warn(missing_docs)]` clean; docs have runnable examples
- [ ] README present
- [ ] Frontend shows errors gracefully (no console crashes)

**Stretch goals (optional)**
- [ ] Add a tray icon or a keyboard shortcut (TAO/window features)
- [ ] Add an async "import from file" command using `tokio::fs`
- [ ] Add a small `unsafe`/FFI feature *with a documented `// SAFETY:` contract* — e.g., read `/proc/self/stat` via raw FFI for a fun "uptime" stat (ties Week 10 together)
- [ ] Publish the stats plugin to crates.io (or just prep it)

---

## After the capstone: making a real contribution

The capstone proves you *can* build this. To prove you can *contribute*, do this when you're ready (any time after Week 12):

1. Pick a project you've used and whose code you've read: **ripgrep**, **Tauri**, **serde**, **clap**, **anyhow**, or a small crate you depend on.
2. Find a **"good first issue"** (or `help wanted` / `E-easy` labels).
3. **Start with docs or tests** — maintainers love these, and they're low-risk. Fix a typo, improve a doc example, add a test for an untested function.
4. Follow `CONTRIBUTING.md` exactly: fork, branch, `cargo fmt`, `cargo clippy`, `cargo test`, then a PR with a clear description.
5. Don't be discouraged by review feedback — a requested change on your first PR is normal and expected. It's how everyone starts.

The skills that make a good contributor are the ones this course drilled: **read the code, run the tests, write a small focused change, document it, and be humble about feedback.**
