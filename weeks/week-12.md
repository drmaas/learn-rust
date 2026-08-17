# Week 12 — The Capstone

**Goal:** Build a real, working Tauri application with a plugin that exercises every core area you've learned — ownership, error handling, iterators, traits, concurrency, async, testing, docs, and reading Tauri's API. This is your "I can contribute to a real Rust project" proof.

The full spec, milestones, and rubric are in **[../capstone/capstone.md](../capstone/capstone.md)**. Read it before starting.

**This week is different:** the daily sessions are *milestones* rather than new concepts. Each milestone is sized for ~30 minutes, but the capstone is meant to be the thing you keep working on — if a milestone takes two sessions, take two. The weekend is your buffer.

---

## Day 1 (Mon) — Milestone 0: Scaffold + design

- Install Tauri deps if you haven't (see SETUP.md).
- Scaffold the app: `sh <(curl https://create.tauri.app/sh)` — vanilla HTML/JS frontend, Rust backend.
- Get the default template running with `cargo tauri dev`.
- **Design first:** open the capstone spec, and before writing code, sketch the module layout and the command list. Write it down in `progress.md`.

## Day 2 (Tue) — Milestone 1: Core library

- Build the pure-Rust core (no Tauri yet): data model, store, validation, search.
- Write tests as you go — this is the part you'll be glad you tested.

## Day 3 (Wed) — Milestone 2: Commands + State + IPC

- Wire the core to Tauri commands (sync + async), manage app state, call from the frontend.
- Handle errors across the IPC boundary — the frontend should see friendly messages, not crashes.

## Day 4 (Thu) — Milestone 3: The plugin

- Extract part of the app into a plugin (the stats plugin in the spec), register it, call it.
- This is your Week 6 + Week 11 work paying off.

## Day 5 (Fri) — Milestone 4: Polish + review

- Frontend UX for errors, `clippy --pedantic` clean, docs, README, final `cargo test`.
- Run the **rubric** in the capstone spec and check off every box.
- Fill in the end-of-course retrospective in `progress.md`.

---

## Weekend — Final review

**Take the quiz:** [Week 12 quiz](../quizzes/week-12.md) — the integrative one, designed to surface weak spots before the capstone. Take it *before* starting the capstone and again *after*.

1. Re-read your `progress.md` from Week 1. Look at your error log. You've come a long way.
2. Run the full rubric one more time.
3. **Optional but strongly recommended:** open a real PR to an open-source Rust project. The capstone spec has a "next steps" section on how to make your first contribution safely.

## After the course

You're not done — you're *started*. The next resources, in order:

1. **Rust for Rustaceans** (Jon Gjengset) — the official "intermediate to advanced" book. Read a chapter a week.
2. **Effective Rust** — 35 idiom items; read one a day as a 10-minute habit.
3. **Crust of Rust** (Jon Gjengset's YouTube) — deep dives on the hard topics you now know enough to appreciate.
4. **Contribute.** The single best way to keep learning is a real PR. Start with docs, then tests, then small fixes.
5. **CodeCrafters** — build a Redis, Git, or SQLite clone in Rust to push into advanced territory.

Welcome to the Rust ecosystem. 🦀
