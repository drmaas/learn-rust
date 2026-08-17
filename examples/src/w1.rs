//! Week 1 — Syntax, types, control flow, Cargo.

/// Shadowing: the inner block shadows `x` with `x * 2`; the outer `x` is
/// unchanged. Expected output: `["inner: 12", "outer: 6"]`.
pub fn shadowing_example() -> Vec<String> {
    let mut out = Vec::new();
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        out.push(format!("inner: {x}"));
    }
    out.push(format!("outer: {x}"));
    out
}

/// `if` is an expression — it produces a value. No ternary needed.
pub fn grade(score: u32) -> &'static str {
    if score >= 90 {
        "A"
    } else {
        "B"
    }
}

/// FizzBuzz via `match` on `(n % 3, n % 5)`.
pub fn fizzbuzz(n: u32) -> String {
    match (n % 3, n % 5) {
        (0, 0) => "FizzBuzz".to_string(),
        (0, _) => "Fizz".to_string(),
        (_, 0) => "Buzz".to_string(),
        _ => n.to_string(),
    }
}

/// Fahrenheit to Celsius (Week 1 Day 2 exercise).
pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

/// `String` + `&str` concatenation: `+` takes a `&str` on the right.
/// (The `let s = ...; s` shape is the course's exact example — kept verbatim.)
#[allow(clippy::let_and_return)]
pub fn string_plus_str() -> String {
    let s = String::from("a") + "b";
    s
}

/// `s.len()` counts BYTES (UTF-8), not characters.
pub fn byte_len(s: &str) -> usize {
    s.len()
}

/// `s.chars().count()` counts characters.
pub fn char_count(s: &str) -> usize {
    s.chars().count()
}

/// Integer division truncates.
pub fn int_div() -> i32 {
    5 / 2
}

/// Float division gives a fraction.
pub fn float_div() -> f64 {
    5.0 / 2.0
}

/// `format!` is like Python f-strings.
pub fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shadowing_outputs_12_then_6() {
        assert_eq!(shadowing_example(), vec!["inner: 12", "outer: 6"]);
    }

    #[test]
    fn if_expression_assigns() {
        assert_eq!(grade(95), "A");
        assert_eq!(grade(50), "B");
    }

    #[test]
    fn fizzbuzz_cases() {
        assert_eq!(fizzbuzz(15), "FizzBuzz");
        assert_eq!(fizzbuzz(3), "Fizz");
        assert_eq!(fizzbuzz(5), "Buzz");
        assert_eq!(fizzbuzz(7), "7");
    }

    #[test]
    fn temperature_conversion() {
        assert_eq!(fahrenheit_to_celsius(32.0), 0.0);
        assert!((fahrenheit_to_celsius(98.6) - 37.0).abs() < 1e-9);
    }

    #[test]
    fn string_plus_str_concatenates() {
        assert_eq!(string_plus_str(), "ab");
    }

    #[test]
    fn bytes_vs_chars() {
        assert_eq!(byte_len("héllo"), 6);
        assert_eq!(char_count("héllo"), 5);
    }

    #[test]
    fn integer_vs_float_division() {
        assert_eq!(int_div(), 2);
        assert_eq!(float_div(), 2.5);
    }

    #[test]
    fn greeting_with_format() {
        assert_eq!(greet("Ada"), "Hello, Ada!");
    }
}
