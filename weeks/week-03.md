# Week 3 — Structs, Enums, Pattern Matching

**Goal:** Model real-world data the Rust way. This is where Rust's type system starts to feel like a superpower — you'll encode *impossible states* out of your programs.

**Facet:** syntax & paradigm (algebraic data types).

---

## Map from your languages

| Rust | Java | Python | TypeScript |
|------|------|--------|-----------|
| `struct` | class (data only) | dataclass | `type`/`interface` |
| `impl` methods | methods | methods | methods |
| `enum` | enum / sealed class | — | discriminated union |
| `Option<T>` | `Optional<T>` | `None` | `T \| undefined` |
| `Result<T,E>` | exceptions | exceptions | thrown error |
| `match` + exhaustiveness | `switch` (no exhaustiveness) | `match` | `switch`/exhaustive `switch` |

**Key idea:** Rust enums are **sum types** — a value is *one of* several variants, and each variant can carry data. Java enums are basically named constants; Rust enums are a whole modeling tool. This is the concept TypeScript users know as discriminated unions, but with compile-time exhaustiveness checking.

---

## Day 1 (Mon) — Structs and methods

**Recall (2 min):** The three ownership rules (from memory!).

**Theory (10 min):**
- A `struct` groups named fields: `struct Book { title: String, pages: u32 }`. No inheritance — composition instead.
- Create: `let b = Book { title: String::from("Rust"), pages: 300 };`
- Methods live in `impl` blocks. `&self` borrows (read), `&mut self` mutates, `self` consumes (moves — the method takes ownership).
- Associated functions (no `self`) are constructors: `Book::new(...)` — like `static` methods.
- `#[derive(Debug)]` lets you print with `{:?}` (like a toString, but automatic).

**Exercise (15 min):**
```rust
#[derive(Debug)]
struct Book {
    title: String,
    pages: u32,
}

impl Book {
    fn new(title: &str, pages: u32) -> Book {
        Book { title: title.to_string(), pages }
    }
    fn reading_minutes(&self, wpm: u32) -> u32 {
        self.pages * 500 / wpm   // rough: ~500 words/page
    }
}
```
Add a `Book` method `is_long(&self) -> bool` (pages > 400). Create two books, print them with `{:?}`, and call all three methods. Predict `reading_minutes` output before running.

**Reflect (3 min):** Why does `reading_minutes` take `&self` and not `self`? (If it took `self`, calling it would *destroy* the book.)

---

## Day 2 (Tue) — Enums

**Recall (2 min):** What's the difference between a method taking `&self` vs `self`?

**Theory (10 min):**
- An enum is a type with a fixed set of *variants*, and variants can carry data:
  ```rust
  enum Message {
      Quit,
      Move { x: i32, y: i32 },
      Write(String),
      ChangeColor(u8, u8, u8),
  }
  ```
- `Option<T>` and `Result<T, E>` are just enums in the standard library. `Option::None` carries nothing; `Some(T)` carries a value.
- **Exhaustiveness:** `match` on an enum must cover *every* variant — the compiler refuses to let you forget a case. This eliminates entire classes of bugs (in Java/Python, forgetting a case silently falls through).
- `if let Some(x) = ...` is shorthand for matching one variant.

**Exercise (15 min):** Model a `Message` enum (like above) and write a function `fn describe(m: &Message) -> String` using `match`. Then write a function that returns `Option<u32>` (e.g., parse a user id from a string) and handle it with `match`, then with `if let`. Try to write a `match` that *omits* a variant and read the error — that error is your friend.

**Reflect (3 min):** How is Rust's `enum` different from Java's `enum`? (Java: named constant. Rust: a type where each variant can hold data, and matches must be exhaustive.)

---

## Day 3 (Wed) — Pattern matching deep dive

**Recall (2 min):** What does "exhaustive" mean for `match`?

**Theory (10 min):** Patterns are a mini-language:
- Literals: `1 => ...`, `"x" => ...`
- Ranges: `1..=5 => ...`
- Wildcards: `_ => ...` (catch-all), `_x` (catch-all but bind)
- Destructuring: pull fields out of structs/tuples/enums: `Message::Move { x, y } => ...`
- `..` ignores the rest: `Point { x, .. }`
- `if let` — match one pattern, ignore the rest
- `let ... else` — match or return early:
  ```rust
  let Some(user) = get_user() else { return; };
  ```
- **Refutable vs irrefutable:** `let x = 5` always succeeds (irrefutable); `let Some(x) = ...` might fail (refutable) — that's why you need `if let`/`match`/`let-else`. The compiler explains this when you get it wrong.

**Exercise (15 min):** Write a function that takes a `(i32, i32)` coordinate and a `Message` and returns a `String` using `match` with: literal arms, a range, a destructured struct variant, and `_`. Then write a function `fn describe_optional(v: Option<u32>) -> String` and implement it three ways — `match`, `if let`, and `let else` — so you see all three idioms.

**Reflect (3 min):** When would you use `match` vs `if let`? (Match: many cases or exhaustiveness matters. `if let`: exactly one case you care about.)

---

## Day 4 (Thu) — Derives, newtypes, and types as documentation

**Recall (2 min):** What's a refutable pattern? Give an example.

**Theory (10 min):**
- `#[derive(...)]` auto-implements common traits: `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`, `Hash`, `Default`, `PartialOrd`, `Ord`. One line replaces boilerplate.
- **Tuple structs:** `struct Point(i32, i32);` — fields by position.
- **The newtype pattern:** wrap a primitive to give it meaning and prevent mix-ups:
  ```rust
  struct UserId(u64);
  struct Age(u8);
  ```
  Now `UserId` and `Age` are different types — you can't accidentally pass an `Age` where a `UserId` is expected (in Java/Python, both would just be `int`/`number`).
- **Types are documentation:** a function signature `fn register(user: UserId) -> Result<(), Error>` tells you more than a comment could.

**Exercise (15 min):** Create `struct UserId(u64)` and `struct Age(u8)` with `#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]`. Write a struct `User { id: UserId, age: Age }`. Try to compare two `UserId`s (needs `PartialEq`), then try to add an `Age` to a `UserId` (should fail — they're different types). Then write a function that takes `&UserId` and `&Age` and try calling it with the wrong order — the compiler catches it.

**Reflect (3 min):** Why is the newtype pattern useful? Give one example of a bug it prevents that Java/Python would let through.

---

## Day 5 (Fri) — Practice: model a domain, write tests

**Recall (2 min):** What does `#[derive(Debug, Clone)]` do?

**Exercise (25 min):** Model a ticket system (this is exercise #3–4 in *100 Exercises to Learn Rust* — you can check your work there):
```rust
#[derive(Debug, Clone, PartialEq)]
enum Status { ToDo, InProgress, Done }

#[derive(Debug, Clone, PartialEq)]
struct Ticket {
    title: String,
    description: String,
    status: Status,
}
```
Requirements:
1. `Ticket::new(title, description, status)` that **validates**: title non-empty and ≤ 50 chars; description non-empty and ≤ 500 chars. Return `Result<Ticket, String>` (or a `TicketError` enum) on failure.
2. Methods `assign`, `start`, `done` that transition status and return errors for invalid transitions (e.g., `ToDo -> Done` should probably fail).
3. Write **unit tests** (`#[test]`) covering: valid ticket, empty title rejected, too-long title rejected, valid transitions, invalid transitions.

Run `cargo test` and make them all pass. Run `clippy --pedantic`.

**Reflect (3 min):** How does encoding status as an enum + validation in the constructor make "impossible states unrepresentable"? (Compare: in Python, a ticket's status is just a string — `"banana"` is a legal status.)

---

## Weekend checkpoint (~15 min)

**Take the quiz:** [Week 3 quiz](../quizzes/week-03.md) — closed-book, then check the answers. Mark your misses in the quiz file.

1. From memory: (a) struct vs enum — when do you use each? (b) what's a newtype and why? (c) what does exhaustive mean? (d) `Option` vs `Result`?
2. Redo Day 2's `Message` enum + `describe` from scratch.
3. Add a new status to your `Status` enum (e.g., `Blocked`) and watch the compiler force you to update every `match` — this is the exhaustiveness superpower in action.
4. Update `progress.md`.

## Reading assignment (10 min)

The Book **chapters 5–6** (structs, enums, pattern matching). Then open one real enum-heavy file: in ripgrep, look at how it models a "match" — or in serde_json, the `Value` enum (`https://docs.rs/serde_json/latest/serde_json/enum.Value.html`) — a textbook example of a data-carrying enum.

## Resources

- The Book, ch. 5–6
- Rustlings: `structs`, `enums`, `option`, `move_semantics` (revisit)
- 100 Exercises to Learn Rust: exercises 1–5 (ticket model)
