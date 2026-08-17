# Week 6 — Traits, Trait Objects, Closures

**Goal:** Master Rust's polymorphism — the trait system — and closures. This week's Friday exercise (a mini plugin system) is a *direct preview* of how Tauri's plugin architecture works, so pay attention.

**Facet:** paradigm (trait-based polymorphism).

---

## Map from your languages

| Rust | Java | Python | TypeScript |
|------|------|--------|-----------|
| `trait` | `interface` | ABC / protocol | `interface` |
| `impl Trait for Type` | `implements` | subclass/mixin | `implements` |
| `dyn Trait` | interface reference | duck typing | `interface` (runtime) |
| generic `T: Trait` | generics + bounds | duck typing | generics + constraints |
| closure | lambda | `lambda` | arrow function |

**Key differences from interfaces:** traits can have *default method implementations* and associated types, and they work with generics at compile time (no runtime cost). Rust has **no inheritance** — traits + generics replace the entire class hierarchy you're used to.

---

## Day 1 (Mon) — Defining and implementing traits

**Recall (2 min):** What's a trait bound? Give the syntax for a generic function bound.

**Theory (10 min):**
```rust
trait Area {
    fn area(&self) -> f64;
    fn describe(&self) -> String {          // default implementation
        format!("area: {:.2}", self.area())
    }
}

struct Circle { radius: f64 }
struct Square { side: f64 }

impl Area for Circle {
    fn area(&self) -> f64 { std::f64::consts::PI * self.radius * self.radius }
}
impl Area for Square {
    fn area(&self) -> f64 { self.side * self.side }
}

fn print_area<T: Area>(shape: &T) {          // generic bound
    println!("{}", shape.describe());
}
```
- A type can implement many traits (that's Rust's substitute for multiple inheritance).
- **Coherence rule:** you can implement a trait for a type only if at least one of them is local to your crate. This prevents two crates from defining the same impl (which would be ambiguous).

**Exercise (15 min):** Define an `Area` trait with a default `describe`, implement it for `Circle`, `Square`, and `Triangle`. Write a generic `print_area<T: Area>`. Then add a `Perimeter` trait and implement it for the same types. Run `clippy`.

**Reflect (3 min):** How is a trait-with-default-methods like and unlike an abstract class in Java? (Like: shared behavior. Unlike: no state/fields in a trait, no inheritance chain.)

---

## Day 2 (Tue) — Trait objects: `dyn Trait` vs generics

**Recall (2 min):** What's a default method?

**Theory (10 min):**
- **Generics** (`T: Area`) → *monomorphized*: a separate copy per type, resolved at compile time, zero runtime cost. But `Vec<T>` must hold one concrete type.
- **Trait objects** (`dyn Area`) → *dynamic dispatch*: one pointer to a value whose type is erased; method calls go through a vtable at runtime. Lets you hold *heterogeneous* types in one collection:
  ```rust
  let shapes: Vec<Box<dyn Area>> = vec![
      Box::new(Circle { radius: 2.0 }),
      Box::new(Square { side: 3.0 }),
  ];
  let total: f64 = shapes.iter().map(|s| s.area()).sum();
  ```
- `Box<dyn Trait>` is a trait object on the heap. `&dyn Trait` is a trait object reference.
- **Object safety:** only traits without generic methods can be `dyn`. The compiler tells you if yours isn't.
- Rule of thumb: prefer generics; reach for `dyn` when you need heterogeneous collections or to store different types in one place (e.g., a plugin registry).

**Exercise (15 min):** Build the heterogeneous `Vec<Box<dyn Area>>` above and sum the areas. Then try to put `Circle` and `Square` in a `Vec<Circle>` (obviously fails) and note *why* `dyn` solves it. Then write a function `fn sum_all(shapes: &[Box<dyn Area>]) -> f64`.

**Reflect (3 min):** When would you use `dyn` instead of generics? (Heterogeneous collections; storing callbacks/plugins; when the concrete type is unknown at compile time.)

---

## Day 3 (Wed) — Key standard-library traits

**Recall (2 min):** What's a trait object, and what's the syntax?

**Theory (10 min):** These appear everywhere — knowing them unlocks reading real code:
- `Display` — human-readable (`{}`). Implement manually.
- `Debug` — programmer-readable (`{:?}`). Derived.
- `From<T>` / `Into<T>` — conversions; `?` uses `From` internally.
- `Default` — `T::default()`.
- `PartialEq`/`Eq`, `PartialOrd`/`Ord` — comparisons (derived).
- `Hash` — for hash map keys (derived).
- `AsRef<T>` — cheap reference conversion (`String: AsRef<str>`).
- `Deref` — smart-pointer dereferencing (Week 8).
- `Iterator` — implement `next()` to make your own iterator! `FromIterator` — what `collect` uses.

**Exercise (15 min):** (1) Implement `From<&str>` for a `Name` newtype. (2) Implement `Iterator` for a `Fibonacci` struct (carry `(a, b)` in state, `next()` returns `Some(a)` and advances) — then use `.take(10).collect::<Vec<u64>>()`. (3) Implement `Display` for `Ticket` from Week 3 so `println!("{}", ticket)` works.

**Reflect (3 min):** Why does `?` need `From`? (So a `std::io::Error` can become your custom error automatically — the conversion is part of the type system.)

---

## Day 4 (Thu) — Closures

**Recall (2 min):** What does implementing `Iterator` require?

**Theory (10 min):**
- Closures are anonymous functions that **capture** variables from their environment:
  ```rust
  let factor = 10;
  let multiply = |x: i32| x * factor;   // captures `factor` by reference
  ```
- Three capture modes, matching ownership:
  - `Fn` — captures by reference (can call multiple times, read-only)
  - `FnMut` — captures by mutable reference (can call multiple times, mutates captured state)
  - `FnOnce` — captures by value/move (can call once)
- `move` forces capture by value — needed to send a closure to another thread.
- Closures as arguments: `fn apply<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 { f(x) }`.
- Function pointers (`fn(i32) -> i32`) are a simpler, non-capturing subset.

**Exercise (15 min):** (1) Write a closure that doubles, use it with `map`. (2) Write a closure that mutates a captured counter (it'll need `FnMut` — make a `Vec` of closures or use `filter` to see the error if you try `Fn`). (3) Write `fn do_twice<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 { f(f(x)) }` and call it. (4) Use `move` to send a closure to a thread (preview of Week 9):
```rust
let data = vec![1, 2, 3];
std::thread::spawn(move || println!("{:?}", data)).join().unwrap();
```

**Reflect (3 min):** Why does Rust distinguish `Fn`/`FnMut`/`FnOnce`? (It's ownership again — the closure's *capture* has the same rules as everything else.)

---

## Day 5 (Fri) — Practice: a mini plugin system (Tauri preview)

**Recall (2 min):** What do `Fn`, `FnMut`, `FnOnce` capture, respectively?

**Exercise (25 min):** Build a trait-based plugin system — this is the *shape* of Tauri's plugin architecture:
```rust
trait Plugin {
    fn name(&self) -> &str;
    fn transform(&self, input: &str) -> String;
}
```
Implement 3 plugins: `Uppercase`, `Reverser`, `WordCount` (returns the input with a " (N words)" suffix, or some text transform). Then:
1. A registry: `struct Registry { plugins: Vec<Box<dyn Plugin>> }` with `register(&mut self, plugin: Box<dyn Plugin>)` and `run_all(&self, input: &str) -> Vec<String>`.
2. Register all three and run them on a sentence.
3. Write unit tests for each plugin and the registry.

If you finish early: add a `Plugin::setup(&mut self) -> Result<(), String>` hook that's called once at startup — this mirrors how Tauri plugins hook into app lifecycle.

**Reflect (3 min):** Why is `Box<dyn Plugin>` (trait object) the right choice for the registry instead of generics? (Heterogeneous collection — you can't store `Vec<Uppercase, Reverser, WordCount>` in one `Vec<T>`.)

---

## Weekend checkpoint (~15 min)

**Take the quiz:** [Week 6 quiz](../quizzes/week-06.md) — closed-book, then check the answers. Mark your misses in the quiz file.

1. From memory: (a) generics vs `dyn Trait` — tradeoffs; (b) the three closure traits; (c) what `move` does; (d) `Display` vs `Debug`.
2. Add a 4th plugin to your registry from scratch (no looking).
3. Convert your plugin registry to also expose a `Vec<Box<dyn Fn(&str) -> String>>` of closures — see how closures and trait objects are the same idea.
4. Update `progress.md`.

## Reading assignment (10 min)

The Book **chapter 10** (generics, traits, lifetimes) and **13** (closures). Then read Tauri's plugin trait docs (`https://v2.tauri.app/develop/plugins/`) — just the Rust-side shape. You now understand the pattern it's built on.

## Resources

- The Book, ch. 10, 13
- Rustlings: `traits`, `closures`
- Effective Rust: items on traits and generics (ch. 3)
