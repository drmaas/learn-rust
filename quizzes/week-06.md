# Week 6 Quiz — Traits, Trait Objects, Closures

**Missed questions (fill in as you go):**

---

## Questions (answer closed-book, in writing)

1. What is a `trait`? How is it like a Java interface, and how is it different?

2. Generics (`T: Trait`) vs trait objects (`dyn Trait`) — what's the tradeoff?

3. What is **object safety**? Can a trait with a generic method be used as `dyn Trait`?

4. `Fn`, `FnMut`, `FnOnce` — what does each capture, and how many times can each be called?

5. What does the `move` keyword do in a closure, and when do you need it?

6. **Predict the output:**
   ```rust
   let x = 10;
   let add = |y: i32| x + y;
   println!("{}", add(5));
   ```

7. What's a **default method** in a trait? Give an example.

8. Why is `Box<dyn Plugin>` (rather than a generic) the right type for a plugin registry holding `Uppercase`, `Reverser`, and `WordCount`?

9. `Display` vs `Debug` — what's the difference, and when do you use each?

10. **Write code:** define a `Perimeter` trait and implement it for a `Square { side: f64 }` struct.

11. **Does this compile? Why or why not?**
   ```rust
   trait Speak {
       fn speak(&self) -> String;
   }
   struct Dog;
   impl Speak for Dog {
       fn speak(&self) -> String { "woof".to_string() }
   }
   fn main() {
       let pets: Vec<Box<dyn Speak>> = vec![Box::new(Dog)];
       println!("{}", pets[0].speak());
   }
   ```

12. **Map from your languages:** Python's duck typing lets any object with a `.speak()` method be called polymorphically. How does Rust achieve the same flexibility, and what extra guarantee does it give you?

---

## Answers & Explanations

1. A `trait` defines a set of methods that types can implement — like an interface. Differences: traits can have **default implementations**, **associated types**, and work with **generics at compile time** (zero runtime cost). Rust has no inheritance; traits + generics replace the whole class hierarchy. Also, coherence: you can implement a trait for a type only if at least one is local to your crate.

2. **Generics** (`T: Trait`) → *monomorphized*: a specialized copy per type, resolved at compile time, fastest possible, but a `Vec<T>` holds one concrete type. **`dyn Trait`** → *dynamic dispatch* through a vtable: one pointer to an erased type, slight runtime cost, but lets you hold **heterogeneous** types in one collection. Rule of thumb: generics by default; `dyn` when you need a collection of different types or a plugin registry.

3. Object safety is the set of rules a trait must satisfy to be usable as `dyn Trait`. A trait with **generic methods** is **not** object-safe (the vtable can't represent an arbitrary type parameter) — the compiler rejects it with an error naming the offending method. `dyn`-able traits have only concrete method signatures (generic *types* in the trait are fine; generic *methods* are not).

4. `Fn` captures **by reference** — callable many times, read-only. `FnMut` captures **by mutable reference** — callable many times, can mutate captured state. `FnOnce` captures **by value** (ownership) — callable **once**. It's ownership again: the closure's capture mode mirrors how it uses the environment. `move` forces capture by value (needed for threads).

5. `move` forces the closure to **take ownership** of captured variables (by value) instead of borrowing them. You need it when the closure must outlive the current scope or run on another thread — e.g., `thread::spawn(move || ...)`. Without `move`, the closure would borrow `data`, which the thread can't safely keep.

6. **`15`.** The closure captures `x` by reference (`Fn`), `add(5)` = `10 + 5`. The closure still works because `x` is still alive.

7. A default method is a method with a body in the trait definition — implementors get it for free and can override it. Example: `trait Area { fn area(&self) -> f64; fn describe(&self) -> String { format!("area: {:.2}", self.area()) } }` — `describe` has a default that uses `area`. It's like an abstract class method, but with no state and no inheritance.

8. Because the registry needs to hold **different concrete types** in one `Vec`. `Vec<Uppercase>`, `Vec<Reverser>`, `Vec<WordCount>` are three different types — you can't put them in one `Vec<T>`. `Vec<Box<dyn Plugin>>` erases the concrete type and stores each as a trait object, so all three fit. This is exactly the pattern Tauri's plugin system uses.

9. **`Display`** is human-readable output (`println!("{}", x)`) — you implement it manually for user-facing types. **`Debug`** is programmer-readable (`println!("{:?}", x)`) — you *derive* it for debugging. `Debug` is also required by `assert_eq!` failure messages and `unwrap()` on `Result`/`Option`.

10.
   ```rust
   trait Perimeter {
       fn perimeter(&self) -> f64;
   }

   struct Square { side: f64 }

   impl Perimeter for Square {
       fn perimeter(&self) -> f64 { self.side * 4.0 }
   }
   ```

11. **Compiles and prints `woof`.** `Dog` implements `Speak`; `Box<dyn Speak>` is a valid trait object (object-safe — no generic methods); `pets[0]` is a `Box<dyn Speak>` and `.speak()` dispatches dynamically to `Dog::speak`.

12. Python's duck typing works at runtime — anything with a `.speak()` method is accepted, and if it doesn't have one you get a runtime `AttributeError`. Rust achieves the same flexibility with **`dyn Trait`**, but the check happens at **compile time** — the compiler verifies the type implements `Speak` before the program runs. Same polymorphism, zero runtime surprises (and no runtime overhead for generic cases).
