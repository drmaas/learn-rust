# Progress Tracker

This file is your daily log and spaced-repetition tracker. **Fill it in at the end of every session** (2–3 sentences). It's the single most effective tool in this course — it forces retrieval (the Feynman technique) and schedules your reviews.

---

## Weekly overview

Check off weeks as you complete them (including the weekend checkpoint).

- [ ] Week 1 — Syntax, types, control flow, Cargo
- [ ] Week 2 — Ownership, borrowing, lifetimes-lite
- [ ] Week 3 — Structs, enums, pattern matching
- [ ] Week 4 — Error handling
- [ ] Week 5 — Collections, iterators, functional style
- [ ] Week 6 — Traits, trait objects, closures
- [ ] Week 7 — Modules, crates, testing, docs
- [ ] Week 8 — Smart pointers, lifetimes deep dive, memory, unsafe-lite
- [ ] Week 9 — Concurrency & async
- [ ] Week 10 — FFI, unsafe, systems programming
- [ ] Week 11 — Reading open source + Tauri deep dive
- [ ] Week 12 — Capstone

---

## Daily log

Copy this block for each session. Keep it short — 3 sentences max per section.

```
### Day: Week __, Day __ (date: ____)

**Recall quiz (2 min):** [write your answer BEFORE reading anything]

**What I built / did:** [1-2 sentences]

**What confused me / errors I hit:** [be specific — this is gold for review]

**Concept in my own words (Feynman):** [explain today's key idea as if to a smart friend]

**Review schedule:** [ ] +1 day   [ ] +3 days   [ ] +7 days   [ ] +14 days
```

---

## Spaced-repetition schedule

When you learn a concept, review it at these intervals by **re-explaining it from memory** or **rewriting a small example**:

| Interval | Action |
|----------|--------|
| +1 day | Re-explain the concept in your own words (no looking) |
| +3 days | Re-explain again; write a 5-line example from memory |
| +7 days | Re-explain; solve a small exercise using it |
| +14 days | Re-explain; teach it to someone (or write a blog-style note) |

**Concepts to track** (add your own as you go):

- [ ] Ownership: one owner, drop on scope exit (W2)
- [ ] Moves vs copies (Copy types) (W2)
- [ ] Borrowing rules: many `&` OR one `&mut` (W2)
- [ ] Slices `&str` / `&[T]` (W2)
- [ ] `Result<T,E>` and `?` (W4)
- [ ] `match` exhaustiveness (W3)
- [ ] Iterator combinators: `map`/`filter`/`fold` (W5)
- [ ] Trait objects `dyn` vs generics (W6)
- [ ] `Fn`/`FnMut`/`FnOnce` closures (W6)
- [ ] `Arc<Mutex<T>>` (W9)
- [ ] `async`/`await` + tokio (W9)
- [ ] `unsafe` and raw pointers (W10)
- [ ] FFI `extern "C"` (W10)

---

## Concept map: my languages → Rust

Fill this in as you learn. Writing the mapping forces transfer.

| Concept | Java | Python | TypeScript | Rust |
|---------|------|--------|-----------|------|
| Immutable binding | `final int x` | — | `const x` | `let x` |
| Mutable binding | `int x` | `x` | `let x` | `let mut x` |
| Optional value | `Optional<T>` | `None` | `T \| undefined` | `Option<T>` |
| Fallible operation | exceptions | exceptions | thrown error | `Result<T,E>` |
| Interface | `interface` | ABC | `interface` | `trait` |
| Class w/o inheritance | `class` | dataclass | `type` | `struct` |
| String | `String` | `str` | `string` | `String` / `&str` |
| List comprehension | streams | `[x for x in ...]` | `.map().filter()` | iterator combinators |
| Switch | `switch` | `match` | `switch` | `match` |
| (add rows as you learn...) | | | | |

---

## Error log (your greatest hits)

Every time a borrow-checker error takes you >5 minutes, copy it here with the fix. After Week 3 you'll see patterns; after Week 8 you'll see how far you've come.

```
### Error: [copy the message]
**Where:** [file/line]
**What I thought was wrong:**
**What was actually wrong:**
**Fix:**
**Date:**
```

---

## Weekly checkpoint template

```
### Week __ Checkpoint (date: ____)

**Recall questions (answer from memory, no looking):**
1.
2.
3.

**Exercise I redid from scratch:** [name]
**Did it go faster than the first time?** [yes/no — if no, review again in 3 days]

**One thing I now understand that I didn't at the start of the week:**
```

---

## End-of-course retrospective (Week 12)

- [ ] I can read a large Rust codebase and trace a feature end-to-end
- [ ] I can write idiomatic Rust with ownership, error handling, traits, iterators
- [ ] I can use `unsafe`/FFI *with justification and safety documentation*
- [ ] I can write async code with tokio
- [ ] I have a tested, documented, `clippy --pedantic`-clean capstone project
- [ ] I have (optionally) opened a PR to an open-source Rust project
