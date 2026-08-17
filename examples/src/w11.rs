//! Week 11 — Reading open source + Tauri.
//!
//! The `#[tauri::command]` attribute requires the full `tauri` crate, which
//! pulls in the framework and needs system webview libraries to build. To keep
//! this example crate fast and dependency-light, the command **logic** is
//! implemented as plain functions and tested here. The attribute itself is
//! framework plumbing verified by Tauri's own test suite.

/// `#[tauri::command] fn greet(name: &str) -> String`
pub fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}

/// `#[tauri::command] async fn slow_thing(ms: u64) -> Result<String, String>`
/// Async commands run on tokio so the main thread / UI stays responsive.
pub async fn slow_thing(ms: u64) -> Result<String, String> {
    tokio::time::sleep(std::time::Duration::from_millis(ms)).await;
    Ok(format!("slept {ms}ms"))
}

/// `#[tauri::command] fn increment(counter: State<'_, Counter>) -> u32`
/// State is shared behind a `Mutex`; commands get it via `State`.
pub struct Counter(std::sync::Mutex<u32>);

impl Default for Counter {
    fn default() -> Self {
        Self::new()
    }
}

impl Counter {
    pub fn new() -> Counter {
        Counter(std::sync::Mutex::new(0))
    }

    pub fn increment(&self) -> u32 {
        let mut c = self.0.lock().unwrap();
        *c += 1;
        *c
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greet_works() {
        assert_eq!(greet("Ada"), "Hello, Ada!");
    }

    #[tokio::test]
    async fn slow_thing_works() {
        assert_eq!(slow_thing(1).await, Ok("slept 1ms".to_string()));
    }

    #[test]
    fn counter_increments_through_state() {
        let c = Counter::new();
        assert_eq!(c.increment(), 1);
        assert_eq!(c.increment(), 2);
    }
}
