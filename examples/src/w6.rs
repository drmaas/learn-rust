//! Week 6 — Traits, trait objects, closures.

pub trait Area {
    fn area(&self) -> f64;

    /// A default method — implementors get it for free, can override it.
    fn describe(&self) -> String {
        format!("area: {:.2}", self.area())
    }
}

pub struct Circle {
    pub radius: f64,
}

pub struct Square {
    pub side: f64,
}

impl Area for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl Area for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

/// Generic function with a trait bound.
pub fn print_area<T: Area>(shape: &T) -> String {
    shape.describe()
}

/// Trait objects let a collection hold different concrete types.
pub fn sum_areas(shapes: &[Box<dyn Area>]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}

pub trait Perimeter {
    fn perimeter(&self) -> f64;
}

impl Perimeter for Square {
    fn perimeter(&self) -> f64 {
        self.side * 4.0
    }
}

/// Implementing `Iterator` by hand — Fibonacci.
pub struct Fibonacci {
    a: u64,
    b: u64,
}

impl Fibonacci {
    pub fn new() -> Fibonacci {
        Fibonacci { a: 0, b: 1 }
    }
}

impl Default for Fibonacci {
    fn default() -> Self {
        Self::new()
    }
}

impl Iterator for Fibonacci {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        let next = self.a;
        self.a = self.b;
        self.b += next;
        Some(next)
    }
}

/// `From<&str>` for a newtype.
pub struct Name(String);

impl From<&str> for Name {
    fn from(s: &str) -> Name {
        Name(s.to_string())
    }
}

impl Name {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// A function taking a closure (`Fn`).
pub fn do_twice<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    f(f(x))
}

/// Closures capture their environment by reference.
pub fn closure_capture() -> i32 {
    let factor = 10;
    let multiply = |x: i32| x * factor;
    multiply(5)
}

/// `move` sends owned data to another thread.
pub fn move_closure_to_thread() -> Vec<i32> {
    let data = vec![1, 2, 3];
    std::thread::spawn(move || data.clone()).join().unwrap()
}

/// Mini plugin system (Week 6 Day 5 / Week 7 library crate exercise).
pub trait Plugin {
    fn name(&self) -> &str;
    fn transform(&self, input: &str) -> String;
}

pub struct Uppercase;
pub struct Reverser;
pub struct WordCount;

impl Plugin for Uppercase {
    fn name(&self) -> &str {
        "uppercase"
    }
    fn transform(&self, input: &str) -> String {
        input.to_uppercase()
    }
}

impl Plugin for Reverser {
    fn name(&self) -> &str {
        "reverser"
    }
    fn transform(&self, input: &str) -> String {
        input.chars().rev().collect()
    }
}

impl Plugin for WordCount {
    fn name(&self) -> &str {
        "wordcount"
    }
    fn transform(&self, input: &str) -> String {
        format!("{} ({} words)", input, input.split_whitespace().count())
    }
}

/// A registry of heterogeneous plugins — hence `Box<dyn Plugin>`.
pub struct Registry {
    pub plugins: Vec<Box<dyn Plugin>>,
}

impl Default for Registry {
    fn default() -> Self {
        Self::new()
    }
}

impl Registry {
    pub fn new() -> Registry {
        Registry {
            plugins: Vec::new(),
        }
    }

    pub fn register(&mut self, plugin: Box<dyn Plugin>) {
        self.plugins.push(plugin);
    }

    pub fn names(&self) -> Vec<&str> {
        self.plugins.iter().map(|p| p.name()).collect()
    }

    pub fn run_all(&self, input: &str) -> Vec<String> {
        self.plugins.iter().map(|p| p.transform(input)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn area_trait_and_default_method() {
        let c = Circle { radius: 2.0 };
        let s = Square { side: 3.0 };
        assert!((c.area() - 4.0 * std::f64::consts::PI).abs() < 1e-9);
        assert_eq!(s.area(), 9.0);
        assert_eq!(print_area(&s), "area: 9.00");
    }

    #[test]
    fn trait_objects_hold_heterogeneous_types() {
        let shapes: Vec<Box<dyn Area>> = vec![
            Box::new(Circle { radius: 1.0 }),
            Box::new(Square { side: 2.0 }),
        ];
        let total = sum_areas(&shapes);
        assert!((total - (std::f64::consts::PI + 4.0)).abs() < 1e-9);
    }

    #[test]
    fn perimeter_trait() {
        let s = Square { side: 3.0 };
        assert_eq!(s.perimeter(), 12.0);
    }

    #[test]
    fn fibonacci_iterator() {
        let fib: Vec<u64> = Fibonacci::new().take(10).collect();
        assert_eq!(fib, vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
    }

    #[test]
    fn from_str_newtype() {
        let n = Name::from("ada");
        assert_eq!(n.as_str(), "ada");
    }

    #[test]
    fn closure_as_argument() {
        assert_eq!(do_twice(|x| x + 1, 5), 7);
    }

    #[test]
    fn closure_captures_environment() {
        assert_eq!(closure_capture(), 50);
    }

    #[test]
    fn move_closure_runs_on_thread() {
        assert_eq!(move_closure_to_thread(), vec![1, 2, 3]);
    }

    #[test]
    fn plugin_registry_runs_all() {
        let mut reg = Registry::new();
        reg.register(Box::new(Uppercase));
        reg.register(Box::new(Reverser));
        reg.register(Box::new(WordCount));
        assert_eq!(reg.names(), vec!["uppercase", "reverser", "wordcount"]);
        let out = reg.run_all("hi there");
        assert_eq!(out[0], "HI THERE");
        assert_eq!(out[1], "ereht ih");
        assert_eq!(out[2], "hi there (2 words)");
    }
}
