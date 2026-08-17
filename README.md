# Learn Rust in 12 Weeks

A personalized, evidence-based course to take you from zero Rust to a **proficient contributor** — the level where you can read a core crate like Tauri's, write idiomatic code, and make real contributions to a major open-source Rust project.

**Your profile (from our conversation):**

- CS degree; strong in **Java, Python, TypeScript**
- Completely new to Rust
- **30 minutes / day** to practice
- Prefers **hands-on** work with theory and syntax explained
- Linux
- 12-week timeline

---

## What "proficient" means here

By the end you should be able to:

1. **Read** a large Rust codebase (Tauri, ripgrep, serde) and trace how it works.
2. **Write** idiomatic Rust: ownership-aware code, error handling with `Result`, iterator-based logic, trait-based polymorphism, tested and documented.
3. **Use** the advanced tools safely: `unsafe`, FFI, async/`tokio`, smart pointers — understanding *why* they exist, not just how.
4. **Contribute**: navigate `cargo`/workspaces, run test suites, follow a contributing guide, and make a pull request.

The capstone in **Week 12** is a real Tauri app with a plugin, which exercises all of these.

---

## How this course is built (read this first)

The structure comes from research on how people actually learn programming languages well. The short version:

- **You test yourself constantly** (retrieval practice beats re-reading).
- **You space out your practice** (each session starts with a recall quiz; each week starts with a review).
- **You interleave topics** (review days mix last week's concepts with this week's).
- **You do deliberate practice** (exercises slightly beyond your comfort zone, with the compiler as your coach).
- **You build on what you know** (every concept is mapped from Java/Python/TS).
- **You write by hand, not by autocomplete** (muscle memory + you learn to read compiler errors).

Full explanation of the science and how it's applied: **[HOW-YOU-LEARN.md](HOW-YOU-LEARN.md)**.

---

## Getting started

1. Follow **[SETUP.md](SETUP.md)** — install the Rust toolchain, editor, and Tauri's Linux dependencies. (~30–60 min, one time.)
2. Read **[HOW-YOU-LEARN.md](HOW-YOU-LEARN.md)** — 10 minutes that will make every session more effective.
3. Open **[progress.md](progress.md)** — this is your daily log and spaced-repetition tracker. Fill it in at the end of every session.
4. Start **[weeks/week-01.md](weeks/week-01.md)**.

---

## The weekly rhythm

Each week has **5 practice days (~30 min)** plus a **weekend checkpoint (~15–20 min)**. You can shift days around; the important rule is *one concept per session* and *never skip the recall quiz*.

**Every 30-minute session:**

| Time | What | Why |
|------|------|-----|
| 0–3 min | **Recall quiz** — write down yesterday's key idea from memory | Retrieval practice; defeats the "illusion of competence" |
| 3–15 min | **Theory + syntax**, mapped to Java/Python/TS | Transfer: build on what you know |
| 15–27 min | **Hands-on exercise** — write code, predict output, break code | Deliberate practice; the compiler is your coach |
| 27–30 min | **Reflect** — 2–3 sentences in `progress.md`; note what confused you | Feynman technique; schedules your next review |

**Every weekend:** a short checkpoint — take that week's [quiz](quizzes/README.md) (closed-book, then check the answers), redo one exercise from scratch, update your concept map.

---

## The 12 weeks at a glance

| Week | Theme | Core areas covered |
|------|-------|--------------------|
| 1 | Syntax, types, control flow, Cargo | Reading/writing Rust, tooling |
| 2 | **Ownership, borrowing, lifetimes-lite** | The heart of Rust; why it's safe |
| 3 | Structs, enums, pattern matching | Modeling data; algebraic types |
| 4 | Error handling | `Result`, `?`, custom errors |
| 5 | Collections, iterators, functional style | Idiomatic data processing |
| 6 | Traits, trait objects, closures | Polymorphism; plugin-shaped code |
| 7 | Modules, crates, testing, docs | Project structure; contribution skills |
| 8 | Smart pointers, lifetimes deep dive, memory, unsafe-lite | What's under the hood |
| 9 | Concurrency & async | Threads, channels, tokio |
| 10 | FFI, unsafe, systems programming | The "core library" skill set |
| 11 | Reading open source + Tauri deep dive | Contributor workflow |
| 12 | **Capstone**: a real Tauri app + plugin | Everything, integrated |

---

## Rules that will save you weeks

1. **Turn on `clippy --pedantic` from day one** and follow its suggestions. It's a free Rust expert reviewing your code.
2. **Type code by hand.** No autocomplete, no AI, no copy-paste while learning. If you can't type it, you haven't internalized it.
3. **Read the error messages.** Rust's compiler errors are famously good — often the fix is spelled out. Read them like a small meditation.
4. **Write tiny programs.** One `main.rs` per concept, not one giant tutorial repo. Throw most of your code away.
5. **Don't use `async` before Week 9.** The ownership model must come first; async adds a second set of rules on top.
6. **Use `String`, `.clone()`, `.unwrap()` generously at first.** You'll refactor later — refactoring is where Rust shines.
7. **When stuck, walk away.** A shower or a walk often solves the borrow-checker problem. Frustration means a knowledge gap, not a personal failure.

---

## Resources you'll reference (all free except two)

- **The Rust Book** (official, free): https://doc.rust-lang.org/book/ — the reference text; read specific chapters as assigned.
- **Rustlings** (interactive exercises, free): `cargo install rustlings` — small exercises tied to Book chapters.
- **Rust by Example** (free): https://doc.rust-lang.org/rust-by-example/
- **100 Exercises to Learn Rust** (Luca Palmieri, free/paid): https://rust-exercises.com — the style this course borrows for its exercises.
- **Rust for Rustaceans** (Jon Gjengset, paid) — *after* the course, this is your next book to reach intermediate→advanced.
- **Effective Rust** (David Drysdale, free online): https://effective-rust.com — idioms and best practices.
- **Google's Comprehensive Rust** (free): https://google.github.io/comprehensive-rust/
- **Crust of Rust** (Jon Gjengset, free YouTube): deep dives on hard topics.
- **Tauri docs**: https://v2.tauri.app — for the capstone.

---

## How to use this repo

```
learn-rust/
├── README.md            ← you are here
├── SETUP.md             ← one-time toolchain + Tauri setup
├── HOW-YOU-LEARN.md     ← the research, and how it's applied
├── progress.md          ← your daily log + spaced-repetition tracker
├── quizzes/
│   ├── README.md        ← how to use the quiz bank (spacing rules)
│   └── week-01.md ... week-12.md   ← one quiz + answer key per week
├── examples/            ← every code example as tested Rust (cargo test)
├── weeks/
│   ├── week-01.md ... week-12.md   ← one file per week
└── capstone/
    └── capstone.md      ← the Week 12 project spec
```

Work through the weeks in order. Don't skip weeks — each one builds on the last. If a week's material feels too easy, do your own stretch exercise; if it feels too hard, spend the weekend redoing the exercises before moving on. **Slow is steady, steady is fast.**

---

## Verified examples: every code example is tested

Every code example in this course — from the week files, the quizzes, HOW-YOU-LEARN.md, and the capstone core — lives as real, compilable Rust in `examples/`, with `#[test]`s that **assert the documented behavior**. Nothing is "trust me, this works".

```bash
cd examples
cargo test     # 102 tests, all green
cargo clippy   # clean; capstone.rs also enforces clippy::pedantic
```

See `examples/README.md` for how the examples map to each week, and how deliberately-broken examples (compile errors) and runtime panics are handled — each broken example is documented with its expected error, the corrected version is tested, and the `RefCell` double-borrow panic from Week 8 is a real `#[should_panic]` test.
