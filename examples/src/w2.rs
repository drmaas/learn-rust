//! Week 2 — Ownership, borrowing, lifetimes-lite.
//!
//! Deliberately-broken examples (compile errors) are shown as comments with
//! the expected error; the corrected version is implemented and tested.

/// The `my_func` ownership example. Passing a `String` **moves** it, so
/// `my_func(s); my_func(s);` fails with `E0382: use of moved value`.
/// `.clone()` is the fix.
pub fn my_func(v: String) -> String {
    v
}

pub fn move_then_clone() -> (String, String) {
    let s = String::from("hello");
    let a = my_func(s.clone());
    let b = my_func(s.clone());
    (a, b)
}

/// `i32` is `Copy`: assigning copies the bits, so `a` stays usable.
pub fn copy_semantics() -> i32 {
    let a = 5;
    let b = a; // copies
    a + b
}

/// A tuple containing a `String` is NOT `Copy` — `let u = t;` moves `t`.
pub fn tuple_with_string_moves() -> (i32, String) {
    let t = (1, "x".to_string());
    let u = t; // moves t into u
    u
}

/// Borrowing with `&mut` lets a function mutate through a reference.
/// (The course's example uses `push_str("!")` verbatim.)
#[allow(clippy::single_char_add_str)]
pub fn shout(s: &mut String) {
    s.push_str("!");
}

/// NLL: a shared borrow ends at its last use, so a mutable borrow is allowed
/// afterward. (Older Rust rejected this.)
pub fn nll_example() -> String {
    let mut s = String::from("hi");
    let r1 = &s;
    let _seen = r1.to_string(); // r1's borrow ends here (last use)
    let r2 = &mut s;
    r2.push_str(" there");
    s
}

/// Slices: return a `&str` up to the first space.
pub fn first_word(s: &str) -> &str {
    match s.find(' ') {
        Some(idx) => &s[..idx],
        None => s,
    }
}

/// Explicit lifetimes: the returned reference lives as long as both inputs.
pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/// Borrowing in a helper (the ownership-diagram program): `print_len(&t)`.
/// (The course deliberately uses `&String` here; `&str` would be more flexible.)
#[allow(clippy::ptr_arg)]
pub fn print_len(s: &String) -> usize {
    s.len()
}

/// The weekend ownership-diagram program: `s`, `t` (clone), `print_len(&t)`,
/// then `let u = t;` moves `t` into `u`.
pub fn ownership_diagram() -> usize {
    let s = String::from("hello");
    let t = s.clone();
    let len = print_len(&t);
    let u = t; // moves t into u
    len + u.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clone_fixes_the_move() {
        let (a, b) = move_then_clone();
        assert_eq!(a, "hello");
        assert_eq!(b, "hello");
    }

    #[test]
    fn copy_types_stay_usable() {
        assert_eq!(copy_semantics(), 10);
    }

    #[test]
    fn tuple_with_string_moves_behavior() {
        assert_eq!(tuple_with_string_moves(), (1, "x".to_string()));
    }

    #[test]
    fn mutable_borrow_mutates() {
        let mut s = String::from("hi");
        shout(&mut s);
        assert_eq!(s, "hi!");
    }

    #[test]
    fn nll_allows_mut_after_last_shared_use() {
        assert_eq!(nll_example(), "hi there");
    }

    #[test]
    fn first_word_variants() {
        assert_eq!(first_word("hello world"), "hello");
        assert_eq!(first_word("hello"), "hello");
        assert_eq!(first_word(""), "");
    }

    #[test]
    fn longest_returns_the_longer() {
        assert_eq!(longest("ab", "abc"), "abc");
        assert_eq!(longest("abcd", "abc"), "abcd");
    }

    #[test]
    fn ownership_diagram_works() {
        assert_eq!(ownership_diagram(), 10);
    }
}
