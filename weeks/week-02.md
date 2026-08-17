# Week 2 — Ownership, Borrowing, Lifetimes-lite

**Goal:** Internalize the single concept that makes Rust Rust: **every value has exactly one owner, and when the owner goes out of scope, the value is dropped.** This is the week that separates "writes Rust" from "thinks in Rust." Expect to fight the compiler — that's the exercise.

**Facet:** paradigm (memory model). Java/Python use garbage collection; Rust uses ownership + RAII (Resource Acquisition Is Initialization — the owner *is* the cleanup).

---

## Map from your languages

- Java/Python: objects live until the GC collects them. You never think about *who owns* a value.
- Rust: the compiler tracks ownership *at compile time*. No GC, no runtime overhead, no use-after-free, no double-free. The borrow checker is a co-author, not an adversary.

**The mental model:** imagine every value has a nametag. Exactly one owner at a time. When the owner's scope ends, the value is destroyed. You can *borrow* (look at or temporarily mutate) a value without owning it — but only under strict rules.

---

## Day 1 (Mon) — Ownership and moves

**Recall (2 min):** `String` vs `&str` — which is owned? (This matters all week.)

**Theory (10 min):** The three ownership rules:
1. Each value has one owner.
2. Only one owner at a time.
3. When the owner goes out of scope, the value is dropped.

Passing a `String` to a function **moves** it — the function becomes the new owner, and the old variable is dead. This is *not* copying, and not pass-by-reference. It's the value itself traveling.

**Exercise (15 min):** Type this and try to compile:

```rust
fn my_func(v: String) {
    println!("{v}");
}

fn main() {
    let s = String::from("hello");
    my_func(s);
    my_func(s); // E0382: use of moved value: `s`
}
```

Read the error **carefully** — it explains the move. Then:
1. Change the parameter to `i32`. Now it compiles! Why? (`i32` is `Copy` — see Day 3.)
2. Fix the `String` version by calling `my_func(s.clone())` and explain what that does.

**Reflect (3 min):** In your own words: what does "moved" mean? Why can't you use `s` after passing it? (Compare: in Java, passing an object to a method doesn't stop you using it afterward — that's the mindset shift.)

---

## Day 2 (Tue) — Borrowing: `&` and `&mut`

**Recall (2 min):** What happens to a `String` when you pass it to a function? What's the fix if you need to use it again?

**Theory (10 min):** Borrowing lets others *use* a value without owning it:
- `&T` — a shared (immutable) reference. Many readers allowed.
- `&mut T` — an exclusive (mutable) reference. One writer only.

**The two rules:**
1. You may have **any number of `&T`** OR **one `&mut T`** — never both at once.
2. References must always be valid (never dangle).

These rules exist to prevent data races and use-after-free — bugs that crash C programs and corrupt Java/Python apps (just silently).

**Exercise (15 min):** Write a function that takes `&String` and prints it. Now try to mutate the string through the `&` (it won't compile — read the error). Fix it with `&mut`:

```rust
fn shout(s: &mut String) {
    s.push_str("!");
}
```

Then try to create a `&` and a `&mut` at the same time and read the error. Finally, note that borrows *end* when last used (NLL — non-lexical lifetimes), so this compiles:

```rust
let mut s = String::from("hi");
let r1 = &s;
println!("{r1}");          // r1's borrow ends here
let r2 = &mut s;           // OK — r1 is no longer in use
r2.push_str(" there");
```

**Reflect (3 min):** Why does Rust allow many `&` or one `&mut`, but not both? (Think: data races. In Java, two threads can mutate the same object with no compile-time protection.)

---

## Day 3 (Wed) — Copy, Clone, Move

**Recall (2 min):** What are the two borrowing rules?

**Theory (10 min):** Why did `i32` work but `String` didn't on Day 1?
- **`Copy` types** (all integers, floats, `bool`, `char`, tuples of `Copy` types): assigning or passing *copies* the bits. The original is still usable. Cheap, no heap.
- **`Clone` types** (`String`, `Vec`, most structs): you must call `.clone()` explicitly to copy. Cloning a `String` copies the heap data (can be expensive).
- **Move** is the default for non-`Copy` types: passing/assigning transfers ownership.
- You can make your own type `Copy` if it's all `Copy` fields: `#[derive(Clone, Copy)]`.

**Exercise (15 min):** For each line, **predict** whether it copies, moves, or errors — then run to check:
```rust
let a = 5;
let b = a;          // ? both usable?
let s1 = String::from("hi");
let s2 = s1;        // ? is s1 usable?
let t = (1, "x".to_string());
let u = t;          // ? tuple with a String inside — Copy or move?
```

Then write a struct, derive `Clone, Copy`, and observe. Finally, refactor a program to *avoid* clones by using references instead.

**Reflect (3 min):** Why does Rust make you write `.clone()` explicitly instead of copying automatically like Java? (Hint: implicit copies of heap data are how Java/Python programs accidentally slow down. Rust makes the cost visible.)

---

## Day 4 (Thu) — Slices and your first lifetimes

**Recall (2 min):** Which types are `Copy`? What does `.clone()` do?

**Theory (10 min):**
- Slices: `&str` is a slice of a `String`; `&[i32]` is a slice of a `Vec<i32>`. A slice is a *view* into a contiguous range — pointer + length.
- `fn first_word(s: &str) -> &str` — returning a reference is natural.
- **Lifetimes (lite):** when a function takes references and returns a reference, the compiler needs to know how long the returned reference is valid. In simple cases it's *elided* (inferred). When it can't guess, it asks you to name it:

```rust
fn longest(x: &str, y: &str) -> &str {   // error: missing lifetime specifier
    if x.len() > y.len() { x } else { y }
}
```

The compiler literally suggests the fix. Add it and take time to understand *why* the compiler couldn't figure it out (either input could be returned, so it can't know which lifetime to pick):

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

Read `'a` as: "the returned reference lives as long as the shorter of x and y."

**Exercise (15 min):** Write `first_word(s: &str) -> &str` that returns the prefix up to the first space. Call it on a `String`. Then write `longest` above, and try to use its result after both inputs go out of scope — read the error (this is your first real lifetime error, and it's a good one).

**Reflect (3 min):** Why can't you return a reference to a value that's local to a function? (It would dangle — the value is dropped when the function returns.)

---

## Day 5 (Fri) — Practice: Rustlings + porting

**Recall (2 min):** What does `'a` mean in a function signature?

**Exercise (25 min):**
1. Do the Rustlings exercises for this week: `move_semantics`, `borrow`, `primitive_types`. `rustlings` runs each exercise's tests and tells you when you pass.
2. Port this Java method to Rust and run into the wall on purpose:
   ```java
   // Java: returns a reference to a field
   String getFirst(List<String> list) { return list.get(0); }
   ```
   In Rust, if you take `&Vec<String>` and return `&str`, it works. But if you try to return a reference to something you *create* inside the function, it fails. Understand *why* (you'd be returning a reference to a value you don't own / that's about to be dropped).

**Reflect (3 min):** Write the ownership rules in your own words *without looking* — all three.

---

## Weekend checkpoint (~15 min)

**Take the quiz:** [Week 2 quiz](../quizzes/week-02.md) — closed-book, then check the answers. Mark your misses in the quiz file.

1. From memory: (a) the three ownership rules; (b) the two borrowing rules; (c) `Copy` vs `Clone` vs `Move`; (d) what `'a` means.
2. **Draw** an ownership diagram of this program — who owns what, and when is each value dropped?
   ```rust
   fn main() {
       let s = String::from("hello");
       let t = s.clone();
       print_len(&t);
       let u = t;   // moves t into u
   }
   fn print_len(s: &String) { println!("{}", s.len()); }
   ```
3. Redo the Day 1 exercise from scratch. Does it go faster?
4. Update `progress.md` (concept map, error log — this week's errors are worth keeping).

## Reading assignment (10 min)

Read The Book **Chapter 4** (Ownership). You've lived it all week; now read it for depth — it will click. Also skim the docs for `String` (`https://doc.rust-lang.org/std/string/struct.String.html`) and notice all the methods that take `&self` vs `self`.

## Resources

- The Book, ch. 4 (ownership, references, slices)
- Rustlings: `move_semantics`, `borrow`, `primitive_types`
- Crust of Rust: "Ownership and Borrowing" (video, if you want a deeper pass)
