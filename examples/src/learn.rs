//! Examples from HOW-YOU-LEARN.md.

/// The ownership example: passing a `String` to a function **moves** it, so
/// calling `my_func(s)` twice fails to compile (`E0382: use of moved value`).
/// `.clone()` is the fix. The corrected version is tested below.
pub fn my_func(v: String) -> String {
    v
}

/// The lifetime example: with two input references the compiler can't infer
/// which one is returned, so it demands an explicit `'a`.
pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ownership_example_uses_clone() {
        let s = String::from("hello");
        // Note: `my_func(s); my_func(s);` would be a compile error (moved value).
        // The fix from the course is `.clone()`:
        let first = my_func(s.clone());
        let second = my_func(s);
        assert_eq!(first, "hello");
        assert_eq!(second, "hello");
    }

    #[test]
    fn longest_returns_the_longer() {
        assert_eq!(longest("abc", "ab"), "abc");
        assert_eq!(longest("ab", "abc"), "abc");
    }
}
