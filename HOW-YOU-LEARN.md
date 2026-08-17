# How You Learn — the research, applied

This course isn't a random sequence of Rust topics. It's built on cognitive-science findings about how people learn skills — and programming languages in particular. Here's the evidence and exactly how each finding shapes the course.

---

## 1. The testing effect (retrieval practice)

**The research:** Re-reading and re-watching material creates an *illusion of competence* — it feels like learning but produces weak, fragile memories. Actively *retrieving* information (testing yourself) is dramatically more effective at building durable memory and transfer. This is one of the most robust findings in cognitive science (Brown, Roediger & McDaniel, *Make It Stick*; the "testing effect").

**How the course applies it:**

- Every session starts with a **3-minute recall quiz** — write down yesterday's key idea *before* looking at anything.
- Every weekend has a **checkpoint quiz** (questions are listed in each week file).
- Exercises are "write the code, then check" rather than "read the code."
- In `progress.md` you **explain concepts in your own words** (the Feynman technique) — if you can't explain it simply, you don't know it yet.

**Why this is uncomfortable:** retrieval feels harder than re-reading. That struggle is the point — it's called a *desirable difficulty*. If a session feels easy, you're probably not learning much.

---

## 2. The spacing effect

**The research:** Distributing practice over time (with gaps) beats cramming, even with identical total time. Memory decays, and the act of *reconstructing* after decay strengthens it. (Yuan et al., 2022; the "spacing effect.")

**How the course applies it:**

- One concept per day, never more (30 minutes is deliberately small).
- Each week's first session recalls the previous week.
- `progress.md` tracks a simple **spaced-repetition schedule**: review a concept after 1 day, then 3 days, then 7 days, then 14 days.
- The weekend checkpoint redoes an exercise *from scratch* — retrieval after a gap.

---

## 3. Interleaving

**The research:** Practicing related skills in a mixed order (interleaved) beats practicing them in blocks, even though it feels harder and slower in the moment. It improves the ability to *discriminate* between similar problems and transfer to new ones.

**How the course applies it:**

- Friday "practice days" mix the week's new concept with earlier ones.
- Weekend checkpoints deliberately mix topics from the last 2–3 weeks.
- The capstone (Week 12) forces you to pull everything together at once.

---

## 4. Deliberate practice

**The research:** Improvement comes from practicing *just beyond* your current ability, with specific goals and immediate feedback — not from doing easy things repeatedly (Ericsson & Pool, *Peak*).

**How the course applies it:**

- Exercises are slightly uncomfortable but achievable within 30 minutes.
- **The compiler is your coach**: it gives immediate, specific feedback. Rust's error messages are famously detailed — often the fix is written out for you.
- `clippy --pedantic` gives constant expert-level feedback from day one.
- If an exercise is trivially easy, the week file tells you to invent a harder version.

---

## 5. The generation effect

**The research:** Producing an answer *before* seeing it (even a wrong answer) improves learning versus just being shown the correct answer.

**How the course applies it:**

- Many exercises are **"predict the output"** — guess what the code prints (or whether it compiles) *before* running it.
- "Write it by hand, then compile" — the errors you make on the way are learning opportunities.
- Exercises tell you to **break working code** and explain the resulting error.

---

## 6. Transfer: building on what you already know

**The research:** New learning is anchored to prior knowledge. Mapping new concepts onto concepts you already understand accelerates learning — as long as you're careful about where the mental models *differ* (this is the advice from corrode.dev's "Flattening Rust's Learning Curve," one of the best practical guides on learning Rust).

**How the course applies it:** Every concept gets a "map from Java/Python/TS" box. Useful (but imperfect) mappings:

| Rust | Java | Python | TypeScript |
|------|------|--------|-----------|
| `struct` | class (data only, no inheritance) | dataclass | interface/type |
| `enum` | sealed class / enum | — | discriminated union |
| `trait` | interface | protocol/ABC | interface |
| `Option<T>` | `Optional<T>` | `None` | `T \| undefined` |
| `Result<T,E>` | exceptions (but it's a *value*) | exceptions | thrown errors |
| `match` | `switch` (more powerful) | `match` | `switch`/`switch(true)` |
| `let` (immutable) | `final` | — | `const` |
| `&str` / `String` | `String` | `str` | `string` |
| iterator combinators | streams | list comprehensions | array methods |

**The flip side — where you must *unlearn*:** Rust is most different exactly where you're most comfortable:

- **Values move**, they don't get copied or GC'd. Passing a `String` to a function *moves* it.
- **Mutability is explicit** (`mut`), not the default.
- **Errors are values** (`Result`), not exceptions you throw and catch.
- **No inheritance.** Composition, traits, and generics replace it.
- **References can't outlive their data** — lifetimes, which Java/GC languages never make you think about.

The course spends Week 2 entirely on the mental-model shift. That's deliberate.

---

## 7. The four facets of a language

**The research:** When you learn a language you're really learning four distinct things (from the "Techniques for Efficiently Learning Programming Languages" guide): (1) **syntax & semantics**, (2) the **paradigm**, (3) the **artifact ecosystem** (build/run/packages), and (4) **tooling**. Confusing them causes frustration — you debug your code when the problem is actually your toolchain.

**How the course applies it:** Each week file tags every concept with which facet it belongs to, and Week 1 explicitly introduces all four (Cargo = ecosystem, `fmt`/`clippy`/rust-analyzer = tooling). When something breaks, ask: *is this a syntax problem, a design problem, a build problem, or a tooling problem?*

---

## 8. Focused vs. diffuse thinking

**The research:** Intense focus explores a narrow region of solution space; diffuse (relaxed) thinking connects ideas across your whole experience. Getting stuck is often solved by *stopping* (Oakley, *A Mind for Numbers*).

**How the course applies it:**

- Rule from the README: **when stuck on the borrow checker, walk away.** A shower or a walk genuinely works.
- Don't burn your 30 minutes smashing your head against one error. Set a 10-minute timer; if you're still stuck, note the error in `progress.md` and stop. Your subconscious will keep working.

---

## 9. Feedback and reading real code

**The research:** Expert performance is built on a large library of *recognized patterns*, which comes from reading lots of high-quality examples, not just writing your own. ("Read lots of other people's code... ripgrep is some of the best Rust code out there." — corrode.dev)

**How the course applies it:** Every week has a **reading assignment** — real code from the standard library, ripgrep, serde, or Tauri. You won't understand all of it at first; that's fine. The goal is pattern recognition, which compounds.

---

## 10. Muscle memory and effortful writing

**The research:** Fluency in a programming language requires typing it without stumbling — it's a physical skill, not just conceptual (corrode.dev calls this "muscle memory" and argues code completion actively blocks it).

**How the course applies it:**

- **Type everything by hand.** No autocomplete while learning (rust-analyzer is for navigation, not writing).
- Predict output *before* running — this builds "will this compile?" intuition.
- You'll make more syntax errors by hand — good. Reading those errors builds intuition for what Rust's error output looks like.

---

## 11. Sleep, exercise, and the body

**The research:** Memory consolidation happens during sleep; exercise improves learning and retention. (This is the most boring and most reliable finding of all.)

**The course's advice:** 30 minutes a day is sustainable *because* it's small. Protect your sleep. The single best "study hack" is going to bed on time after a practice day.

---

## What the research says *not* to do

- ❌ Re-read the Book cover to cover without writing code. (Illusion of competence.)
- ❌ Watch tutorials passively. (Same problem — you feel like you're learning.)
- ❌ Use AI/autocomplete to write your exercises. (corrode.dev: "Don't go on auto-pilot" — you strengthen bad habits and never build the mental model. You can't explain what you didn't write.)
- ❌ Learn a new domain *and* a new language at once. (This is why the course has you *port* programs you already know rather than build novel apps — until the capstone.)
- ❌ Cram. 30 minutes a day for 12 weeks beats 6-hour weekends.

---

## The one mindset shift that matters most

From corrode.dev's *Flattening Rust's Learning Curve* — the single most important piece of advice in all this research:

> **Let your guard down.** Treat the borrow checker as a co-author, not an adversary. Junior devs with no baggage often learn Rust faster than senior engineers who "know how programming works." When you fight the compiler, it's usually exposing a design flaw in your code, not being pedantic.

You'll hit walls in Week 2 (ownership) and Week 3 (lifetimes). Everyone does. The wall *is* the lesson. Keep going.
