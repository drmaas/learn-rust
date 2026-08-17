# Week 2 Quiz — Ownership, Borrowing, Lifetimes-lite

**Missed questions (fill in as you go):**

---

## Questions (answer closed-book, in writing)

1. State the **three ownership rules** from memory.

2. **Does this compile? Why or why not?**
   ```rust
   fn main() {
       let s = String::from("hi");
       let t = s;
       println!("{s}");
   }
   ```

3. State the **two borrowing rules** from memory.

4. For each of these types, say whether assignment/passing **copies** or **moves**: `i32`, `bool`, `String`, `Vec<i32>`, a tuple `(i32, i32)`.

5. **Predict the output** (does it even compile?):
   ```rust
   let mut s = String::from("a");
   let r1 = &s;
   println!("{r1}");
   let r2 = &mut s;
   r2.push('b');
   println!("{s}");
   ```

6. Why can't a function return a reference to a value it created locally?

7. What does `'a` mean in `fn longest<'a>(x: &'a str, y: &'a str) -> &'a str`?

8. Why does passing an `i32` to a function *not* "move" it, but passing a `String` does?

9. What is a **slice**? What is `&str` a slice of?

10. **Write code:** a function `first_word(s: &str) -> &str` that returns everything up to the first space (or the whole string if no space).

11. In Java, `list.get(0)` returns a reference to an element and you can keep using it freely. Why is the Rust equivalent (`fn get_first(list: &Vec<String>) -> &str`) constrained in a way Java's isn't?

12. **Map from your languages:** Python's garbage collector frees objects "eventually." What frees a Rust value, and how is the timing different?

---

## Answers & Explanations

1. **(1)** Each value has exactly one owner. **(2)** Only one owner at a time. **(3)** When the owner goes out of scope, the value is dropped.

2. **Does not compile** — `E0382: use of moved value: 's'`. `let t = s;` *moves* the `String` into `t`; `s` is now invalid. Fixes: use `&s` (borrow), `s.clone()` (copy), or don't use `s` afterward.

3. **(1)** You may have any number of `&T` **or** exactly one `&mut T` — never both at once. **(2)** References must always be valid (never dangle).

4. `i32` → **copies** (`Copy`). `bool` → **copies**. `String` → **moves** (not `Copy`). `Vec<i32>` → **moves**. `(i32, i32)` → **copies** (a tuple of `Copy` types is `Copy`). A tuple containing a `String` would *move*.

5. **Compiles, prints `a` then `ab`.** Thanks to **NLL** (non-lexical lifetimes), `r1`'s borrow ends after its last use (`println!`), so the `&mut` borrow is allowed afterward. This is the modern behavior — older Rust would have rejected it.

6. Because the local value is **dropped when the function returns** — the reference would dangle, pointing at freed memory. The borrow checker prevents this at compile time. (In Java/Python this would be a silent bug or require the GC to keep it alive.)

7. `'a` is a **lifetime parameter**. Read it as: "the returned reference is valid as long as *both* `x` and `y` are valid" (specifically, as long as the shorter of the two). The compiler couldn't guess which input is returned, so you name the relationship.

8. `i32` implements the **`Copy` trait** — passing it copies the bits, and the original stays usable. `String` is not `Copy` (it owns heap data; copying would be expensive), so passing it *moves* ownership. Copy is free and implicit; moves are also free — they just transfer ownership instead of duplicating.

9. A slice is a **view into a contiguous range of data** — a pointer + a length. `&str` is a slice of a `String`'s bytes (guaranteed valid UTF-8). `&[i32]` is a slice of a `Vec<i32>`. Slices let you borrow part of a collection without owning or copying it.

10.
   ```rust
   fn first_word(s: &str) -> &str {
       match s.find(' ') {
           Some(idx) => &s[..idx],
           None => s,
       }
   }
   ```
   (Or with a loop: `for (i, c) in s.char_indices() { if c == ' ' { return &s[..i]; } } s` — note `char_indices` because `' '` is 1 byte but indexing by bytes is only safe at char boundaries.)

11. Java's `list.get(0)` hands you a reference with **no lifetime information** — the reference can outlive the list (or the list can change), causing bugs or needing GC to keep things alive. Rust's `get_first` must return a reference whose lifetime is tied to the input — the compiler guarantees the returned `&str` can't outlive the `Vec` it came from. Same operation, but Rust makes the relationship explicit and safe.

12. Python's GC frees an object when the *collector* notices it's unreachable — nondeterministic. Rust drops a value **when its owner goes out of scope** — deterministic, exactly once, no collector needed. This is RAII: the owner *is* the cleanup. That's why Rust has no GC and still leaks nothing (unless you ask it to).
