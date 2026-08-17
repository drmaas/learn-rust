//! Integration test for the Week 6 / Week 7 plugin system.
//!
//! This is the Week 7 "library crate" exercise in action: tests in `tests/`
//! use the crate's PUBLIC API exactly as an external consumer would.

use course_examples::w6::{Registry, Reverser, Uppercase, WordCount};

#[test]
fn registry_public_api_from_outside() {
    let mut reg = Registry::new();
    reg.register(Box::new(Uppercase));
    reg.register(Box::new(Reverser));
    reg.register(Box::new(WordCount));

    let results = reg.run_all("hi");
    assert_eq!(results[0], "HI");
    assert_eq!(results[1], "ih");
    assert_eq!(results[2], "hi (1 words)");
}

#[test]
fn registry_empty_has_no_plugins() {
    let reg = Registry::new();
    assert!(reg.names().is_empty());
    assert!(reg.run_all("x").is_empty());
}
