# Week 12 Quiz — Integrative (Capstone Prep)

This quiz is different: it **pulls everything together**. Each question combines concepts from multiple weeks — the way the capstone does. Take it *before* starting the capstone to find your weak spots, then again *after* to confirm you're ready.

**Missed questions (fill in as you go):**

---

## Questions (answer closed-book, in writing)

1. **Design:** why split the capstone into a pure-Rust `clipnote-core` library (no Tauri imports) plus a separate Tauri app? Name two concrete benefits.

2. **Design + Week 9:** the capstone stores notes in `AppState { store: Mutex<Store> }`. Why `Mutex` and not `RefCell`?

3. **Write code (Weeks 4 + 11):** a `#[tauri::command]` `get_note` that takes `state: State<'_, AppState>` and an `id: u64`, and returns `Result<Note, String>` — converting the store's error into a friendly string.

4. **Write code (Weeks 5 + 3):** a function that takes `&[Note]` and returns the most common word across all note bodies as `Option<(String, u32)>`, using a `HashMap` and iterators.

5. **True or false (Weeks 7 + 12):** the core library should have unit tests, and the Tauri app should have integration tests for the commands.

6. **Design (Weeks 6 + 11):** the stats plugin computes word frequency. Why make it a plugin (separate crate) instead of just another function in the app? Name the architectural benefit.

7. **Week 9 + 11:** you make `search_notes` an `async` command that calls the core's search. What must be true of the types crossing the async boundary, and why?

8. **Week 4 + 11:** a command returns `Result<Note, String>`. Why `String` for the error rather than the core's `ClipError` enum? What's the tradeoff?

9. **Week 8 + 11:** the core `Store` holds a directory path and writes files. Why is `Store` *not* wrapped in `Arc` here, but the `Mutex`-guarded state is shared via `State`? (Think about who owns what.)

10. **Write code (Weeks 2 + 3 + 5):** `Note::new(title: &str, body: &str) -> Result<Note, ClipError>` — validate title non-empty and ≤ 120 chars. Write the validation with a `match`/guard and return a descriptive error.

11. **Week 10 + 12 (stretch):** the capstone's stretch goal adds an FFI read of `/proc/self/stat`. What's the *safest* way to structure that so the app's safe code never touches `unsafe`?

12. **Reflection (all weeks):** list the *one* concept from each of Weeks 1–11 that you'd still struggle to explain to a Java developer. (This is your personal review list — there's no right answer, but there's a right *habit*.)

---

## Answers & Explanations

1. **(1) Testability:** the core is pure Rust — `cargo test` runs instantly with no windowing system, no webview, no Tauri runtime. You test the logic (validation, storage, search) in isolation. **(2) Decoupling:** the core doesn't depend on the framework, so it's portable (usable from a CLI, a test, or another framework), and the app layer is thin glue. This is exactly how Tauri itself separates `tauri` from `tauri-runtime` from the platform crates.

2. `Mutex` is the **thread-safe** version of the borrow-checker-at-runtime idea. Tauri commands — especially `async` ones — run on tokio's **multi-threaded runtime**, so `State` can be accessed from different threads. `RefCell` is explicitly single-threaded (`!Sync`) and would fail to compile. `Mutex` gives the same interior mutability but with thread safety. (Week 8 taught you the concept; Week 9 taught you why `Mutex` over `RefCell`.)

3.
   ```rust
   #[tauri::command]
   fn get_note(state: State<'_, AppState>, id: u64) -> Result<Note, String> {
       let store = state.store.lock().map_err(|e| e.to_string())?;
       store
           .get(id)
           .map_err(|e| e.to_string())?
           .ok_or_else(|| format!("note {id} not found"))
   }
   ```
   `?` unwraps; the store's `ClipError` and the lock's `PoisonError` are converted to `String`; `ok_or_else` turns the `Option<Note>` into an error for the not-found case. The frontend sees a rejected Promise with the message.

4.
   ```rust
   fn most_common_word(notes: &[Note]) -> Option<(String, u32)> {
       let mut counts: HashMap<String, u32> = HashMap::new();
       for note in notes {
           for word in note.body.split_whitespace() {
               *counts.entry(word.to_lowercase()).or_insert(0) += 1;
           }
       }
       counts.into_iter().max_by_key(|(_, count)| *count).map(|(w, c)| (w, c))
   }
   ```
   `entry().or_insert(0) += 1` (Week 5), `max_by_key` (Week 5 combinators), `Option` return for the empty case (Week 3).

5. **True.** The core library gets thorough **unit tests** (validation, store round-trip, search) — fast, no Tauri needed. The app gets **integration tests** in `tests/` for the command layer (setting up a `tauri::test` harness or testing the command functions with constructed `State`). This mirrors Week 7's unit-vs-integration split.

6. **Separation of concerns + reusability.** A plugin is a self-contained crate with its own API and capability permissions — it can be versioned, documented, tested, and reused across apps independently, and it keeps the app crate small. It also exercises the exact architecture real Tauri plugins use, so you're practicing the "contributor" skill. The cost: a bit more setup (plugin builder, registration) for a feature that could be a plain function — the benefit is architectural, not functional.

7. The types crossing the async boundary must be **`Send`** (and the future must be `Send + 'static`) so tokio can move the task between threads. Practically: the command takes owned/borrowed inputs that are `Send`, calls the core's search (which is sync and fast), and returns serializable results. `State<'_, T>` works in async commands because Tauri handles the `Send` requirements for you — but your custom types (e.g., anything holding `Rc` or raw pointers) would break it. This is Week 9's `Send`/`Sync` rule showing up in real code.

8. `String` is the **pragmatic choice for the IPC boundary** — it serializes to JSON cleanly and the frontend just displays it. The tradeoff: you lose the *structured* error (the frontend can't programmatically distinguish "not found" from "io error" without string-matching). If the frontend needs to branch on error type, return a serializable error enum instead (e.g., `{ "kind": "NotFound", "message": "..." }`). For a small app, `String` is fine; for a library, keep the enum. (Week 4's philosophy: errors are values — decide how structured they need to be.)

9. `Store` is a single object **owned by the app** — it's created once in `main` and moved into `app.manage(...)`. There's no need for `Arc` because there's exactly one owner and no one else holds a clone. The `Mutex` inside it exists because *commands* need to borrow it mutably from multiple threads — `Mutex` provides that shared mutable access. `Arc` would only be needed if multiple independent owners each needed their own handle to the same `Store` (they don't — `State` gives commands access to the one managed instance). This is Week 8's ownership model: one owner, shared access via `State` + `Mutex`, no `Arc` needed.

10.
   ```rust
   pub fn new(title: &str, body: &str) -> Result<Note, ClipError> {
       if title.trim().is_empty() {
           return Err(ClipError::InvalidNote("title cannot be empty".into()));
       }
       if title.chars().count() > 120 {
           return Err(ClipError::InvalidNote("title too long (max 120 chars)".into()));
       }
       Ok(Note {
           id: 0, // assigned by the store
           title: title.to_string(),
           body: body.to_string(),
           created_at: std::time::SystemTime::now(),
       })
   }
   ```
   Validation returns `Result` (Week 4), `chars().count()` not `len()` for character count (Week 1), `String::from`/`.to_string()` (Week 1), struct construction (Week 3). "Impossible states unrepresentable": a `Note` can't exist with an empty title.

11. Confine the FFI to a **small, dedicated module** with the safe-wrapper pattern (Week 10): one function `read_proc_stat() -> Result<String, ClipError>` that owns the only `unsafe` block, does the raw read, converts to a `String`, and documents its invariants in a `// SAFETY:` comment. The rest of the app calls the **safe** function. Bonus: wrap it so the unsafe is behind a single well-tested seam, and (even better) gate it behind a feature flag so it's optional. The goal is that a reviewer can audit *one* small function for safety.

12. **No single right answer** — but the habit matters. If you can't write a clear one-sentence explanation for a concept, that's your next review target. Good candidates people usually flag: lifetimes (W2/W8), `dyn` vs generics (W6), `Send`/`Sync` (W9), interior mutability (W8), and why `unsafe` is needed at all (W10). Whatever you listed — go re-read that week's file and its quiz, then re-answer here in a week.
