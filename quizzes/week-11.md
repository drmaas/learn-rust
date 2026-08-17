# Week 11 Quiz — Reading Open Source + Tauri Deep Dive

**Missed questions (fill in as you go):**

---

## Questions (answer closed-book, in writing)

1. Order Tauri's stack from top to bottom: `tauri-runtime-wry`, `tauri`, `tao`, `wry`, `tauri-runtime`.

2. What does `#[tauri::command]` do?

3. Sync vs async commands — when should a command be `async`?

4. How do you share state with your commands? (Two pieces: registering it, and injecting it.)

5. What are the **three jobs** of a Tauri plugin?

6. What does `tauri::generate_handler![cmd1, cmd2]` do?

7. What are **capabilities/permissions** for in Tauri v2?

8. In which crate does most of Tauri's `unsafe` live, and why?

9. What's the first thing you should read when entering an unfamiliar codebase?

10. What is a **"good first issue"** label, and why do projects have it?

11. **Write code:** a `#[tauri::command]` that takes a name and returns a greeting.

12. **Map from your languages:** how is calling a Tauri command from JS (`invoke`) like calling an async function in TypeScript, and what's the Rust-side contract you must satisfy?

---

## Answers & Explanations

1. **`tauri`** (top) → **`tauri-runtime`** (abstract glue) → **`tauri-runtime-wry`** (concrete impl) → **`wry`** (webview binding) → **`tao`** (windowing). `tauri` is the user-facing crate; `tao`/`wry` are the upstream crates that talk to the OS. Each layer depends on the one below it.

2. `#[tauri::command]` turns a plain Rust function into a **command callable from the frontend** via `invoke`. The macro (in `tauri-macros`/`tauri-codegen`) generates the IPC plumbing: argument deserialization, return serialization, and error mapping. You register commands with `.invoke_handler(tauri::generate_handler![...])`.

3. A command should be **`async`** when it does slow I/O or waits — the runtime runs it on tokio via `async_runtime::spawn`, so the main thread (and UI) stays responsive. Sync commands run on the main thread and are for fast work. Rule of thumb: anything that sleeps, reads from disk/network, or could take more than a few milliseconds → `async`.

4. **(1) Register:** `app.manage(MyState { ... })` in the builder — this stores the state in the app. **(2) Inject:** a command takes `state: State<'_, MyState>` as a parameter and the `State` gives you access. This is Tauri's dependency injection — commands get shared state without globals.

5. A plugin: **(1)** enables Rust code to do "something" (functionality), **(2)** provides glue to integrate into an app (setup hooks, lifecycle), and **(3)** provides a **JavaScript API** for the frontend to call. Plugins are how Tauri's feature set is extended — and they're just Rust crates using the `Plugin` trait (the pattern you built in Week 6).

6. `tauri::generate_handler![cmd1, cmd2]` **registers a list of commands** with the invoke system — it's how the app knows which functions the frontend is allowed to call. Pass it to `.invoke_handler(...)`. The macro generates the dispatch table that routes `invoke("cmd1", args)` to the right function.

7. **Security.** Capabilities/permissions declare, per window or plugin, **which commands the frontend may invoke**. Tauri's IPC is capability-gated — the webview can't call arbitrary Rust functions, only the ones you explicitly permit. This limits the blast radius if the frontend is compromised (e.g., XSS). In v2 this is configured in the `capabilities/` directory.

8. **`wry`** (and to a degree `tao`) — because they talk directly to the **operating system webview/windowing APIs** (WebKitGTK on Linux, WebView2 on Windows, WKWebView on macOS), which are C/Obj-C APIs. Crossing into them requires raw pointers and FFI (`unsafe`). The higher layers (`tauri`, `tauri-runtime`) are almost entirely safe — that's the safe-wrapper pattern in action.

9. **`README.md`** (what it is), then **`ARCHITECTURE.md`** if present (how it's structured), then the **tests** (what it promises), then trace **one feature end-to-end** from its entry point using rust-analyzer go-to-definition. Don't read linearly — follow one path from input to output.

10. A "good first issue" is a **small, well-scoped bug or enhancement** with clear instructions, curated by maintainers for newcomers. Projects use them to onboard contributors safely (low risk, teaches the contribution workflow) and to grow their contributor base. Starting there — especially with a docs or test fix — is the recommended entry ramp.

11.
   ```rust
   #[tauri::command]
   fn greet(name: &str) -> String {
       format!("Hello, {name}!")
   }
   ```
   Registered with `.invoke_handler(tauri::generate_handler![greet])`, called from JS with `invoke('greet', { name: 'Ada' })`.

12. `invoke('cmd', args)` from JS returns a **Promise** — just like calling an `async` function in TypeScript: `const result = await invoke('cmd', { ... })`. The Rust-side contract: the command's arguments must **deserialize from the JSON you pass** (types must match), and its return value must **serialize back** (or be a `Result` — `Err` becomes a rejected Promise). So the type system on both sides must agree, and errors cross the boundary as rejections rather than panics.
