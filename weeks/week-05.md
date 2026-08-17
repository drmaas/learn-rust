# Week 5 — Collections, Iterators, Functional Style

**Goal:** Write idiomatic data-processing code. This is where Rust stops feeling like "Java with different syntax" and starts feeling like *Rust* — iterator chains are the heart of the language.

**Facet:** paradigm (functional style).

---

## Map from your languages

| Rust | Java | Python | TypeScript |
|------|------|--------|-----------|
| `Vec<T>` | `ArrayList<T>` | `list` | `T[]` |
| `HashMap<K,V>` | `HashMap` | `dict` | `Map` |
| `HashSet<T>` | `HashSet` | `set` | `Set` |
| iterator: `map`/`filter`/`fold` | streams | list comprehensions | `.map()`/`.filter()`/`.reduce()` |
| `collect()` | `.toList()` | `list(...)` | `Array.from(...)` |

**The key idea:** iterators are **lazy** — nothing happens until you consume them (`for`, `collect`, `sum`, `fold`). Chains of `map`/`filter` describe a transformation; the work happens once, in one pass.

---

## Day 1 (Mon) — Collections

**Recall (2 min):** What's the difference between `String` and `Vec<u8>`? (A `String` is a `Vec<u8>` that's guaranteed valid UTF-8.)

**Theory (10 min):**
- `Vec<T>`: `push`, `pop`, `get(i)` (returns `Option<&T>` — no out-of-bounds panics if you use `get`), `len`, `contains`, `remove`, `sort`.
- `HashMap<K,V>`: `insert`, `get` (returns `Option<&V>`), `remove`, `contains_key`, and the **`entry` API**:
  ```rust
  let mut counts: HashMap<String, u32> = HashMap::new();
  *counts.entry("apple".to_string()).or_insert(0) += 1;
  ```
  `entry` is the idiomatic "get or insert" — cleaner than the Python `dict.setdefault` dance.
- `HashSet<T>` — a `HashMap<K, ()>`; `insert`, `contains`, set ops (`union`, `intersection`).
- `BTreeMap`/`BTreeSet` — sorted versions (like Java's `TreeMap`).

**Exercise (15 min):** Build a tiny inventory: `HashMap<String, u32>` of item → count. Add items with `entry().or_insert(0) += 1`, look up with `get`, remove with `remove`, and print all items sorted by name. Then use a `HashSet` to dedupe a list of words.

**Reflect (3 min):** Why does `get` return `Option<&V>` instead of panicking on a missing key? (It's the "errors are values" philosophy — absence is a normal case, not a crash.)

---

## Day 2 (Tue) — Iterators: `iter`, `into_iter`, `iter_mut`

**Recall (2 min):** What does `entry().or_insert(0)` do?

**Theory (10 min):** Three ways to iterate a `Vec`, and they differ in *ownership*:
- `v.iter()` — borrows; yields `&T`. The vector is still usable after.
- `v.iter_mut()` — mutably borrows; yields `&mut T`. You can change elements.
- `v.into_iter()` — **consumes** the vector; yields owned `T`. After this, `v` is gone (moved).
- `for x in v` desugars to `v.into_iter()` — so `for x in v` consumes `v`! If you want to read without consuming, use `&v` or `v.iter()`.
- `collect()` gathers the iterator into a collection — the target type is inferred from context: `let squares: Vec<i32> = (0..10).collect();`

**Exercise (15 min):**
```rust
let v = vec![1, 2, 3];
for x in &v { println!("{x}"); }   // borrows — v still usable
println!("{}", v.len());           // OK

let w = vec![10, 20, 30];
for x in w { println!("{x}"); }    // consumes w
// println!("{}", w.len());        // ERROR: use of moved value
```
Then write a function that takes `&Vec<i32>` and uses `.iter()` to print doubled values, and another that takes `&mut Vec<i32>` and uses `.iter_mut()` to double in place. Predict which compiles and which doesn't.

**Reflect (3 min):** Why does `for x in v` consume but `for x in &v` not? (Ownership: `into_iter` takes the value; `iter` borrows it.)

---

## Day 3 (Wed) — Combinators

**Recall (2 min):** `iter()` vs `into_iter()` — which consumes?

**Theory (10 min):** The workhorses:
- `map(f)` — transform each item
- `filter(pred)` — keep items where pred is true
- `fold(init, f)` — accumulate (like `reduce`)
- `sum()`, `product()`, `count()`, `min()`, `max()`
- `take(n)`, `skip(n)`, `zip(other)`, `enumerate()`, `chain(other)`, `flat_map(f)`
- `filter_map(f)` — filter + map + unwrap `Option` in one step

**Porting a Python list comprehension:**
```python
# Python
[x*x for x in range(10) if x % 2 == 0]
```
```rust
// Rust
let squares: Vec<i32> = (0..10).filter(|x| x % 2 == 0).map(|x| x * x).collect();
```

**Exercise (15 min):** Compute stats on a `Vec<f64>` using combinators only (no loops): mean (`sum`/`len`), min, max, count of values above the mean, and a `HashMap<String, u32>` of word frequencies from a sentence (split on whitespace, `fold` or `entry`). Then port two more of your Python list comprehensions.

**Reflect (3 min):** Why are combinator chains better than `for` loops here? (Readability, one pass, no manual index bookkeeping, and they compose.)

---

## Day 4 (Thu) — Generics

**Recall (2 min):** What does `collect()` do, and how does Rust know what to collect into?

**Theory (10 min):**
- Generic functions work for many types:
  ```rust
  fn largest<T: PartialOrd>(list: &[T]) -> &T {
      let mut largest = &list[0];
      for item in list {
          if item > largest { largest = item; }
      }
      largest
  }
  ```
- `T: PartialOrd` is a **trait bound** — "T must be comparable." The compiler checks this at compile time.
- Generic structs: `struct Wrapper<T> { value: T }`.
- **Monomorphization:** the compiler generates a *separate specialized copy* of the generic code for each concrete type used. That's why generic Rust is as fast as hand-written code (unlike Java's erasure + boxing).
- `where` clauses keep long bounds readable: `fn f<T, U>(...) where T: Display, U: Clone { ... }`.

**Exercise (15 min):** Write `largest<T: PartialOrd>` and test it on `&[i32]` and `&[&str]`. Then write a generic `struct Pair<T, U>` with a method `fn first(&self) -> &T`. Then write `fn print_slice<T: std::fmt::Debug>(v: &[T])` and call it with several types. Try calling `largest` on a `Vec<bool>` — `bool` doesn't implement `PartialOrd` — and read the error (it names the missing trait).

**Reflect (3 min):** What does "monomorphization" mean, and why does it make Rust generics fast?

---

## Day 5 (Fri) — Practice: data processing pipeline

**Recall (2 min):** What's a trait bound, and what does `<T: PartialOrd>` mean?

**Exercise (25 min):** Build a mini log analyzer:
1. Create a `log.txt` with lines like `2026-08-01 INFO user logged in` (make 20–30 lines with a few levels: INFO/WARN/ERROR).
2. Read it into a `String`.
3. Parse into `Vec<LogEntry>` where `LogEntry { level: String, msg: String }`.
4. Using combinators: count errors, list distinct levels, find the most common word across all messages (word-frequency `HashMap`), and print a summary.

Write the whole thing with iterator chains — the only `for` loop allowed is the final printing. Run `clippy --pedantic` and fix everything.

**Reflect (3 min):** Which part felt most like "writing Python"? Which part couldn't exist in Python? (The type-driven parsing and the borrow-checked ownership of the file's data.)

---

## Weekend checkpoint (~15 min)

**Take the quiz:** [Week 5 quiz](../quizzes/week-05.md) — closed-book, then check the answers. Mark your misses in the quiz file.

1. From memory: (a) `iter` vs `into_iter` vs `iter_mut`; (b) three combinators and what they do; (c) what is `collect`; (d) what is monomorphization.
2. Redo the log analyzer from scratch (make a new, smaller log file so you can't copy).
3. Add a new requirement (e.g., "errors per day") using combinators only.
4. Update `progress.md`.

## Reading assignment (10 min)

The Book **chapters 8, 10, 13**. Then read iterator-heavy real code: open ripgrep's source (or rayon) and find a `map`/`filter`/`fold` chain — or the `std` docs for `Iterator` (`https://doc.rust-lang.org/std/iter/trait.Iterator.html`) and skim the list of methods. Notice how many there are; you'll use ~10 of them constantly.

## Resources

- The Book, ch. 8, 10, 13
- Rustlings: `vecs`, `iterators`, `generics`, `hashmap`
- 100 Exercises to Learn Rust: exercises 9–15 (iterators/collections)
