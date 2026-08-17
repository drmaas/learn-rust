# Week 11 — Reading Open Source + Tauri Deep Dive

**Goal:** Read a large real codebase like a contributor, and understand Tauri's architecture from the inside. This week is reading-heavy, but every reading task ends with a *hands-on verification* — you run the code, trace it, break it.

**Facet:** all four — you're now using the whole skill set.

---

## Day 1 (Mon) — Navigating a large codebase

**Recall (2 min):** What's the safe-wrapper pattern for FFI? (Last week's core idea.)

**Theory (10 min):** How to find your way in an unfamiliar Rust repo:
- Start with `README.md` and `ARCHITECTURE.md` (if present).
- `cargo doc --open` for docs; docs.rs is pre-built for crates.
- **rust-analyzer**: go-to-definition, find-references, "peek definition" — your superpowers for tracing.
- Read **tests as specs** — a crate's tests tell you what the code promises.
- Find the **entry point** (a `main.rs` or a `pub fn` in `lib.rs`) and follow one feature end-to-end.
- `git log --oneline` to see how the project evolves; `git blame` to see why a line is the way it is.
- Don't read linearly. Trace *one* path: input → processing → output.

**Exercise (15 min):** Clone **ripgrep** (`git clone https://github.com/BurntSushi/ripgrep`) — some of the best Rust code in existence. Trace one feature: the `--count` flag. Find where CLI args are parsed (clap), where the search happens, and where counts are printed. Use rust-analyzer go-to-definition the whole way. Then run `cargo test` (it's a big build — be patient).

**Reflect (3 min):** What was the entry point you started from? How many hops did it take to reach the core search logic?

---

## Day 2 (Tue) — Tauri architecture

**Recall (2 min):** What's the first thing you read in an unfamiliar repo?

**Theory (10 min):** Tauri's crate map (this is the "core library" you aimed at):
- **`tauri`** — the top-level crate: config, commands, app lifecycle, everything glued together.
- **`tauri-runtime`** — the abstract interface between Tauri and the webview layer.
- **`tauri-runtime-wry`** — the concrete implementation on top of **WRY**; systems-level interactions (printing, monitors).
- **`tauri-macros`** / **`tauri-codegen`** — proc macros: `#[tauri::command]`, config parsing, asset embedding.
- **`tauri-utils`** — shared utilities (config parsing, CSP, platform detection).
- **`tauri-build`** — build-time macro application.
- **Upstream crates:** **TAO** (windowing — a fork of winit) and **WRY** (the webview binding; this is where the `unsafe` lives, Week 10).
- **Plugins:** Rust crate + JS package; three jobs — Rust functionality, glue, and a JS API.

**Exercise (15 min):** Clone Tauri (`git clone https://github.com/tauri-apps/tauri`) — or just browse on GitHub. Find `ARCHITECTURE.md`. Then: find the definition of the `#[tauri::command]` attribute macro (`tauri-macros`), and trace what `invoke` does — search for `invoke` from the JS side to the Rust handler. You're tracing the exact path a frontend call takes.

**Reflect (3 min):** Which crate would you expect to contain the most `unsafe`? (WRY — it talks to the OS webview directly. That's where the safety-critical FFI lives.)

---

## Day 3 (Wed) — Tauri commands & state (hands-on)

**Recall (2 min):** What does `tauri-runtime-wry` do?

**Theory (10 min):** Time to write Tauri code. Key concepts:
- `#[tauri::command]` — turn a Rust function into a callable from the frontend.
- **Sync commands** run on the main thread; **async commands** (`async fn`) run on tokio via `async_runtime::spawn` — use async for anything slow.
- **State:** `app.manage(MyState)` stores shared state; `State<'_, MyState>` injects it into a command.
- The frontend calls with `invoke('command_name', { args })` from the JS API.
- `tauri::Builder` wires everything: `.invoke_handler(tauri::generate_handler![cmd1, cmd2])` and `.manage(state)`.

**Exercise (15 min):** Scaffold a real app:
```bash
sh <(curl https://create.tauri.app/sh)   # or: cargo create-tauri-app
# choose: vanilla HTML/JS (no framework needed) + Rust
cd <app>
cargo tauri dev
```
Then add commands:
```rust
#[tauri::command]
fn greet(name: &str) -> String { format!("Hello, {name}!") }

#[tauri::command]
async fn slow_thing(ms: u64) -> Result<String, String> {
    tokio::time::sleep(std::time::Duration::from_millis(ms)).await;
    Ok(format!("slept {ms}ms"))
}

struct Counter(std::sync::Mutex<u32>);

#[tauri::command]
fn increment(counter: State<'_, Counter>) -> u32 {
    let mut c = counter.0.lock().unwrap();
    *c += 1;
    *c
}
```
Register them, call from the JS side, and watch it work in the window.

**Reflect (3 min):** Why are async commands run on tokio rather than blocking the main thread? (The UI would freeze — same reason you don't block the JS event loop.)

---

## Day 4 (Thu) — Building a Tauri plugin

**Recall (2 min):** How do you inject state into a command?

**Theory (10 min):** Tauri plugins (v2) are crates that hook into the app lifecycle and expose commands. A plugin has:
- A Rust crate: `tauri::plugin::Plugin` trait with a `setup` hook, plus commands registered via a `Builder`.
- An optional JS package exposing an API.
- **Permissions/capabilities** in v2 — you declare which commands the frontend may call (this is a security feature; Tauri's IPC is capability-gated).

**Exercise (15 min):** Build a minimal "greeter" plugin:
1. `cargo new --lib tauri-plugin-greeter`
2. Add `tauri = { version = "2", features = [] }` and implement the plugin builder pattern (follow the [official plugin guide](https://v2.tauri.app/develop/plugins/)).
3. Register the plugin in your app: `.plugin(tauri_plugin_greeter::init())`.
4. Add a command to the plugin and call it from the frontend.

**Reflect (3 min):** How is a Tauri plugin like your Week 6 `Plugin` trait system? (Same shape: a trait, a registry, hooks. You built the pattern 5 weeks ago.)

---

## Day 5 (Fri) — Practice: the contribution workflow

**Recall (2 min):** What are the three jobs of a Tauri plugin?

**Exercise (25 min):** Run the real contributor workflow on a real project:
1. Pick a project: Tauri, ripgrep, or a smaller crate you've used (serde, anyhow, clap).
2. Read its `CONTRIBUTING.md` (or the "Contributing" section of the README).
3. Clone it, build it, run the test suite. (Set expectations: big projects take a while to compile — that's normal.)
4. Find a **"good first issue"** (GitHub label) — read the issue and the linked code.
5. Write a **failing test** for what you'd want to fix (or reproduce the bug in a test). You don't have to submit anything — just get to the point where a real fix would be a small step.

**Reflect (3 min):** What's the scariest part of contributing to a big project? What would make you feel ready to actually open a PR? (Log it — we'll address it in the capstone.)

---

## Weekend checkpoint (~15 min)

**Take the quiz:** [Week 11 quiz](../quizzes/week-11.md) — closed-book, then check the answers. Mark your misses in the quiz file.

1. From memory: (a) draw Tauri's crate stack from memory (tauri → runtime → wry/tao); (b) sync vs async commands; (c) the three jobs of a plugin.
2. Rebuild the greeter plugin from scratch in a fresh app.
3. Update `progress.md`.

## Reading assignment (10 min)

Read the Tauri **plugin guide** and **calling-Rust** pages (`https://v2.tauri.app/develop/plugins/`, `https://v2.tauri.app/develop/calling-rust/`). Then read `ARCHITECTURE.md` in the Tauri repo. Skim the `CONTRIBUTING.md` of the project you picked on Day 5.

## Resources

- Tauri docs: https://v2.tauri.app
- Tauri `ARCHITECTURE.md`: https://github.com/tauri-apps/tauri/blob/dev/ARCHITECTURE.md
- ripgrep: https://github.com/BurntSushi/ripgrep
