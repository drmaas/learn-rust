//! Week 5 — Collections, iterators, functional style.

use std::collections::{HashMap, HashSet};

/// `entry().or_insert(0) += 1` — the idiomatic "count occurrences" pattern.
pub fn word_frequencies(text: &str) -> HashMap<String, u32> {
    let mut counts: HashMap<String, u32> = HashMap::new();
    for word in text.split_whitespace() {
        *counts.entry(word.to_string()).or_insert(0) += 1;
    }
    counts
}

/// `for x in &v` borrows, so `v` is still usable afterward.
pub fn borrow_then_use() -> usize {
    let v = vec![1, 2, 3];
    let mut sum = 0;
    for x in &v {
        sum += x;
    }
    sum + v.len()
}

/// `iter_mut()` changes elements in place.
pub fn double_in_place(v: &mut [i32]) {
    for x in v.iter_mut() {
        *x *= 2;
    }
}

/// Python `[x*x for x in range(10) if x % 2 == 0]` -> Rust.
pub fn even_squares() -> Vec<i32> {
    (0..10).filter(|x| x % 2 == 0).map(|x| x * x).collect()
}

/// Combinator stats: mean, min, max, count above the mean.
pub fn stats(nums: &[f64]) -> (f64, f64, f64, usize) {
    let mean = nums.iter().sum::<f64>() / nums.len() as f64;
    let min = nums.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = nums.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let above = nums.iter().filter(|x| **x > mean).count();
    (mean, min, max, above)
}

/// Generic function with a trait bound.
pub fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

/// Generic struct.
pub struct Pair<T, U> {
    pub first: T,
    pub second: U,
}

impl<T, U> Pair<T, U> {
    pub fn first(&self) -> &T {
        &self.first
    }
    pub fn second(&self) -> &U {
        &self.second
    }
}

/// Generic over `Debug` for printing.
pub fn format_slice<T: std::fmt::Debug>(v: &[T]) -> String {
    format!("{v:?}")
}

/// Log analyzer (Week 5 Day 5).
#[derive(Debug, Clone, PartialEq)]
pub struct LogEntry {
    pub level: String,
    pub msg: String,
}

/// Parse `2026-08-01 INFO user logged in` lines.
pub fn parse_log(text: &str) -> Vec<LogEntry> {
    text.lines()
        .filter_map(|line| {
            let mut parts = line.splitn(3, ' ');
            let _date = parts.next()?;
            let level = parts.next()?;
            let msg = parts.next().unwrap_or("");
            Some(LogEntry {
                level: level.to_string(),
                msg: msg.to_string(),
            })
        })
        .collect()
}

pub fn count_errors(entries: &[LogEntry]) -> usize {
    entries.iter().filter(|e| e.level == "ERROR").count()
}

pub fn distinct_levels(entries: &[LogEntry]) -> HashSet<String> {
    entries.iter().map(|e| e.level.clone()).collect()
}

/// Most common word across all messages.
pub fn most_common_word(entries: &[LogEntry]) -> Option<(String, u32)> {
    let mut counts: HashMap<String, u32> = HashMap::new();
    for e in entries {
        for w in e.msg.split_whitespace() {
            *counts.entry(w.to_lowercase()).or_insert(0) += 1;
        }
    }
    counts.into_iter().max_by_key(|(_, c)| *c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entry_or_insert_counts() {
        let counts = word_frequencies("apple apple banana");
        assert_eq!(counts.get("apple"), Some(&2));
        assert_eq!(counts.get("banana"), Some(&1));
        assert_eq!(counts.get("pear"), None);
    }

    #[test]
    fn borrow_does_not_consume() {
        assert_eq!(borrow_then_use(), 9); // (1+2+3) + 3
    }

    #[test]
    fn iter_mut_doubles_in_place() {
        let mut v = vec![1, 2, 3];
        double_in_place(&mut v);
        assert_eq!(v, vec![2, 4, 6]);
    }

    #[test]
    fn list_comprehension_translated() {
        assert_eq!(even_squares(), vec![0, 4, 16, 36, 64]);
    }

    #[test]
    fn combinator_stats() {
        let (mean, min, max, above) = stats(&[1.0, 2.0, 3.0, 4.0]);
        assert_eq!(mean, 2.5);
        assert_eq!(min, 1.0);
        assert_eq!(max, 4.0);
        assert_eq!(above, 2);
    }

    #[test]
    fn largest_is_generic() {
        assert_eq!(*largest(&[3, 1, 2]), 3);
        assert_eq!(*largest(&["a", "ccc", "bb"]), "ccc");
    }

    #[test]
    fn pair_accessors() {
        let p = Pair {
            first: 1,
            second: "x".to_string(),
        };
        assert_eq!(*p.first(), 1);
        assert_eq!(p.second(), "x");
    }

    #[test]
    fn format_slice_debug() {
        assert_eq!(format_slice(&[1, 2]), "[1, 2]");
    }

    #[test]
    fn log_analysis_counts_and_levels() {
        let log = "2026-08-01 INFO user logged in\n\
                   2026-08-01 ERROR boom\n\
                   2026-08-01 WARN slow\n\
                   2026-08-01 ERROR crash";
        let entries = parse_log(log);
        assert_eq!(entries.len(), 4);
        assert_eq!(count_errors(&entries), 2);
        assert_eq!(distinct_levels(&entries).len(), 3);
    }

    #[test]
    fn most_common_word_found() {
        let log = "2026-08-01 INFO hello world\n2026-08-01 INFO hello again\n";
        let entries = parse_log(log);
        assert_eq!(most_common_word(&entries), Some(("hello".to_string(), 2)));
    }
}
