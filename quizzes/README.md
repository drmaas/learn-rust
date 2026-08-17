# Quiz Bank

A reusable question bank for all 12 weeks, built on the same science as the course: **retrieval practice** and **spaced repetition**. The questions are harder than "re-read the chapter" — they make you produce answers, predict output, and write code from memory. That's the point.

Each week file has two sections:
1. **Questions** — answer these closed-book, in writing.
2. **Answers & Explanations** — check after you've answered. Explanations tell you *why*, so a wrong answer becomes a lesson.

---

## How to use it

**When:** Take each week's quiz on the **weekend checkpoint** (or Friday), ~15–20 minutes. Then revisit *missed* questions on the spaced-repetition schedule below.

**The rules (from HOW-YOU-LEARN.md):**
1. **No peeking.** Read the questions, write your answers *before* scrolling to the answers. The retrieval — even a wrong retrieval — is what builds memory.
2. **Predict before you run.** For every "predict the output" question, write your guess down, then check. Don't run the code first.
3. **Write, don't recite.** Typing answers forces the same muscle memory as typing code.
4. **Mark your misses.** Keep a tally at the top of each quiz file (e.g., `Missed: Q3, Q7`) so your future self knows what to revisit.

**Spaced-repetition schedule for missed questions:**

| Interval | Action |
|----------|--------|
| +1 day | Re-answer missed questions from memory |
| +3 days | Re-answer again; for write-code questions, retype the solution |
| +7 days | Re-answer; if you still miss one, re-read that week's section first |
| +14 days | Re-answer; teach the concept to someone (or write it down) |

**When you miss something, always ask:** *which week covered this, and what was my mental model wrong about?* Then go back to that week's file. The quizzes are a diagnostic, not a score.

---

## Question types you'll see

- **Concept recall** — "What are the three ownership rules?" (The foundation.)
- **Predict the output** — code + "what prints?" (Builds "will this compile?" intuition.)
- **Does this compile?** — code + "why or why not?" (The single highest-value question type in Rust.)
- **Write code** — "Write a function that…" (Deliberate practice; type it out.)
- **Map from your languages** — "How is a Rust `enum` different from a Java `enum`?" (Transfer.)

---

## The bank at a glance

| Quiz | Covers | Question count |
|------|--------|----------------|
| [week-01](week-01.md) | Syntax, types, control flow, Cargo | 12 |
| [week-02](week-02.md) | Ownership, borrowing, lifetimes-lite | 12 |
| [week-03](week-03.md) | Structs, enums, pattern matching | 12 |
| [week-04](week-04.md) | Error handling | 12 |
| [week-05](week-05.md) | Collections, iterators, functional style | 12 |
| [week-06](week-06.md) | Traits, trait objects, closures | 12 |
| [week-07](week-07.md) | Modules, crates, testing, docs | 12 |
| [week-08](week-08.md) | Smart pointers, lifetimes, memory | 12 |
| [week-09](week-09.md) | Concurrency & async | 12 |
| [week-10](week-10.md) | FFI, unsafe, systems programming | 12 |
| [week-11](week-11.md) | Reading OSS + Tauri | 12 |
| [week-12](week-12.md) | Integrative (capstone prep) | 12 |

**Total: 144 questions.**

---

## Quick-start

1. Finish Week 1 → on the weekend, open [week-01.md](week-01.md) and take the quiz.
2. Mark your misses. Revisit them at +1/+3/+7/+14 days.
3. Repeat each week. By Week 12, you'll have a personal map of everything you've retained — and everything to review before the capstone.
