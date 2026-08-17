# Week 5 Quiz — Collections, Iterators, Functional Style

**Missed questions (fill in as you go):**

---

## Questions (answer closed-book, in writing)

1. `iter()`, `into_iter()`, `iter_mut()` — what does each yield, and which one *consumes* the collection?

2. What does `collect()` do, and how does Rust know what to collect into?

3. **Write code:** translate this Python to Rust:
   ```python
   [x*x for x in range(10) if x % 2 == 0]
   ```

4. What does `counts.entry("apple".to_string()).or_insert(0) += 1;` do?

5. **Predict the output** (does it compile?):
   ```rust
   let v = vec![1, 2, 3];
   for x in &v { print!("{x}"); }
   println!(" {}", v.len());
   ```

6. What is **monomorphization**? Why are Rust generics as fast as hand-written code?

7. Briefly: what do `map`, `filter`, and `fold` each do?

8. **Predict:** what is `(0..10).filter(|x| x % 2 == 0).count()`?

9. Why does `HashMap::get` return `Option<&V>` instead of panicking on a missing key?

10. `HashMap` vs `BTreeMap` — what's the practical difference?

11. **Write code:** using iterators only (no loops), compute the sum of squares of a `&[i32]`.

12. **Map from your languages:** Java's `list.stream().map(...).filter(...).collect(Collectors.toList())` — what's the Rust equivalent shape, and what's lazy about it?

---

## Answers & Explanations

1. `iter()` → **`&T`** (borrows; collection still usable). `iter_mut()` → **`&mut T`** (mutably borrows; you can change elements). `into_iter()` → **`T`** (owned; **consumes** the collection — it's moved). `for x in v` desugars to `v.into_iter()` and consumes `v`; use `for x in &v` to borrow.

2. `collect()` gathers the iterator's items into a collection. Rust knows the target type from **context** — usually the type annotation: `let squares: Vec<i32> = (0..10).collect();` or the function's return type. `collect` can build `Vec`, `HashMap`, `HashSet`, `String`, and more (via `FromIterator`).

3.
   ```rust
   let squares: Vec<i32> = (0..10).filter(|x| x % 2 == 0).map(|x| x * x).collect();
   ```
   Note the order: `filter` then `map` — same as the comprehension (which reads "for x, if even, square it").

4. It looks up `"apple"`; if absent, **inserts `0`** and returns a mutable reference to the entry's value; then `+= 1` increments it. This is the idiomatic "count occurrences" pattern — one line, no get-then-set dance. (`entry` returns an `Entry` enum; `or_insert` gives you the value to mutate.)

5. **Compiles, prints `123 3`.** `for x in &v` borrows `v`, so `v.len()` afterward is fine. If it were `for x in v`, the vector would be consumed and `v.len()` would be a compile error.

6. **Monomorphization** = the compiler generates a separate, specialized copy of generic code for each concrete type used (`Vec<i32>` and `Vec<String>` get different code). No runtime dispatch, no boxing, no type erasure — the generic code is as fast as if you'd written it by hand for each type. (Java erases generics to `Object` + casts; Rust doesn't.)

7. `map(f)` transforms each item (`T → U`). `filter(pred)` keeps items where `pred` is true. `fold(init, f)` accumulates: `items.fold(0, |acc, x| acc + x)` sums — it carries state across items (like `reduce`).

8. **`5`.** The evens in `0..10` are 0, 2, 4, 6, 8. `count()` consumes the iterator and returns how many items passed the filter.

9. Because "key not present" is a **normal, expected case**, not a crash. Returning `Option<&V>` forces you to handle absence explicitly and lets you chain with `unwrap_or`, `map`, etc. Panicking on a missing key would make every lookup a potential crash.

10. `HashMap` is unordered (fast, hash-based) — great when you don't care about order. `BTreeMap` keeps keys **sorted** (ordered, tree-based) — use it when you need to iterate in key order or find ranges. Same API otherwise.

11.
   ```rust
   fn sum_of_squares(nums: &[i32]) -> i32 {
       nums.iter().map(|x| x * x).sum()
   }
   ```
   (`sum()` is `fold` with addition and `0` as the identity — it exists for numeric types.)

12. Shape: `list.iter().map(...).filter(...).collect::<Vec<_>>()`. The **laziness** is the key difference: in Rust the iterator chain does *nothing* until something consumes it (`collect`, `sum`, a `for` loop). Java streams are also lazy, but in Rust the laziness is explicit — a `.map()` with no consumer does zero work, and the compiler will even warn about it.
