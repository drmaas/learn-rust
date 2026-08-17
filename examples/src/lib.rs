//! # Verified course examples
//!
//! Every code example from the **Learn Rust in 12 Weeks** course, implemented
//! as real, compilable Rust with `#[test]`s that assert the documented
//! behavior. Run everything with `cargo test` from this directory.
//!
//! Modules map to course material:
//! - [`learn`] — examples from HOW-YOU-LEARN.md
//! - [`w1`] .. [`w12`] — one module per course week
//! - [`capstone`] — the Week 12 ClipNote core library (Note, Store, search)
//!
//! ## How deliberately-broken examples are handled
//!
//! The course includes examples that are *supposed* to fail to compile (e.g.
//! using a moved value, or creating a `&` and `&mut` at once). Those can't be
//! ordinary tests, so each one is:
//! - shown as a comment with the expected compiler error, and
//! - the **corrected** version is implemented and tested here.
//!
//! Runtime panics that the course asks you to trigger (e.g. the `RefCell`
//! double-borrow) are real `#[should_panic]` tests — the panic is the point.

pub mod capstone;
pub mod learn;
pub mod w1;
pub mod w10;
pub mod w11;
pub mod w2;
pub mod w3;
pub mod w4;
pub mod w5;
pub mod w6;
pub mod w7;
pub mod w8;
pub mod w9;
