//! Week 9 — Concurrency & async.

use std::sync::{mpsc, Arc, Mutex};
use std::thread;

/// Basic thread spawn + join.
pub fn thread_spawn_join() -> i32 {
    let handle = thread::spawn(|| 40 + 2);
    handle.join().unwrap()
}

/// `move` gives the thread owned data.
pub fn thread_move_value() -> Vec<i32> {
    let data = vec![1, 2, 3];
    let handle = thread::spawn(move || data.clone());
    handle.join().unwrap()
}

/// mpsc channel: multiple producers, single consumer. The receiver's `for`
/// loop ends once all senders are dropped.
pub fn channel_multi_producer() -> Vec<i32> {
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();
    thread::spawn(move || tx.send(1).unwrap());
    thread::spawn(move || tx2.send(2).unwrap());
    let mut out: Vec<i32> = rx.iter().collect();
    out.sort_unstable();
    out
}

/// `Arc<Mutex<T>>` — shared mutable state across threads.
pub fn arc_mutex_counter(increments: u32) -> u32 {
    let counter = Arc::new(Mutex::new(0u32));
    let mut handles = Vec::new();
    for _ in 0..increments {
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let mut v = c.lock().unwrap();
            *v += 1;
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    // Bind the guard to a variable: a temporary in a tail expression would
    // otherwise live until the end of the block, outliving `counter`.
    let final_value = *counter.lock().unwrap();
    final_value
}

/// Producer/consumer pipeline: each worker sends a result over a channel.
pub fn producer_consumer_workers(n: u32) -> Vec<u32> {
    let (tx, rx) = mpsc::channel();
    let mut handles = Vec::new();
    for i in 0..n {
        let tx = tx.clone();
        handles.push(thread::spawn(move || {
            tx.send(i * i).unwrap();
        }));
    }
    drop(tx); // drop the original so rx closes once workers finish
    let mut out: Vec<u32> = rx.iter().collect();
    for h in handles {
        h.join().unwrap();
    }
    out.sort_unstable();
    out
}

/// The same job with `Arc<Mutex<Vec>>` shared state instead of a channel.
pub fn shared_state_vec(items: Vec<u32>) -> Vec<u32> {
    let shared = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::new();
    for item in items {
        let shared = Arc::clone(&shared);
        handles.push(thread::spawn(move || {
            shared.lock().unwrap().push(item);
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    let mut out = shared.lock().unwrap().clone();
    out.sort_unstable();
    out
}

/// async/await: an `async fn` returns a `Future`; nothing runs until driven.
pub async fn add(a: u32, b: u32) -> u32 {
    a + b
}

pub fn block_on_add() -> u32 {
    futures::executor::block_on(add(2, 3))
}

pub async fn chain(a: u32) -> u32 {
    let x = add(a, 1).await;
    add(x, 1).await
}

pub fn block_on_chain() -> u32 {
    futures::executor::block_on(chain(5))
}

/// tokio tasks (lightweight, multiplexed).
pub async fn tokio_squares() -> Vec<u32> {
    let mut handles = Vec::new();
    for i in 0..10 {
        handles.push(tokio::spawn(async move {
            tokio::time::sleep(std::time::Duration::from_millis(1)).await;
            i * i
        }));
    }
    let mut squares = Vec::new();
    for h in handles {
        squares.push(h.await.unwrap());
    }
    squares
}

/// `select!` races two futures and runs whichever completes first.
pub async fn select_race() -> &'static str {
    let slow = tokio::time::sleep(std::time::Duration::from_millis(50));
    let fast = tokio::time::sleep(std::time::Duration::from_millis(1));
    tokio::select! {
        _ = slow => "slow",
        _ = fast => "fast",
    }
}

/// Parallel word counter — threads version (Week 9 Day 5).
pub fn parallel_word_count_threads(files: &[&str]) -> usize {
    let (tx, rx) = mpsc::channel();
    let mut handles = Vec::new();
    for file in files {
        let tx = tx.clone();
        let path = file.to_string();
        handles.push(thread::spawn(move || {
            let count = std::fs::read_to_string(&path)
                .map(|text| text.split_whitespace().count())
                .unwrap_or(0);
            tx.send(count).unwrap();
        }));
    }
    drop(tx);
    let total: usize = rx.iter().sum();
    for h in handles {
        h.join().unwrap();
    }
    total
}

/// Parallel word counter — async version with `tokio::fs`.
pub async fn parallel_word_count_async(files: &[&str]) -> usize {
    let mut handles = Vec::new();
    for file in files {
        let path = file.to_string();
        handles.push(tokio::spawn(async move {
            match tokio::fs::read_to_string(&path).await {
                Ok(text) => text.split_whitespace().count(),
                Err(_) => 0,
            }
        }));
    }
    let mut total = 0;
    for h in handles {
        total += h.await.unwrap();
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thread_spawn_join_works() {
        assert_eq!(thread_spawn_join(), 42);
    }

    #[test]
    fn thread_move_works() {
        assert_eq!(thread_move_value(), vec![1, 2, 3]);
    }

    #[test]
    fn channel_delivers_all_messages() {
        assert_eq!(channel_multi_producer(), vec![1, 2]);
    }

    #[test]
    fn arc_mutex_counter_increments() {
        assert_eq!(arc_mutex_counter(8), 8);
        assert_eq!(arc_mutex_counter(0), 0);
    }

    #[test]
    fn producer_consumer_pipeline() {
        assert_eq!(producer_consumer_workers(4), vec![0, 1, 4, 9]);
    }

    #[test]
    fn shared_state_vec_collects() {
        let mut v = shared_state_vec(vec![3, 1, 2]);
        v.sort_unstable();
        assert_eq!(v, vec![1, 2, 3]);
    }

    #[test]
    fn async_add_and_chain() {
        assert_eq!(block_on_add(), 5);
        assert_eq!(block_on_chain(), 7);
    }

    #[tokio::test]
    async fn tokio_spawn_tasks_return_in_order() {
        assert_eq!(
            tokio_squares().await,
            vec![0, 1, 4, 9, 16, 25, 36, 49, 64, 81]
        );
    }

    #[tokio::test]
    async fn select_races_and_fast_wins() {
        assert_eq!(select_race().await, "fast");
    }

    #[test]
    fn parallel_word_count_threads_and_async_agree() {
        use tempfile::tempdir;
        let dir = tempdir().unwrap();
        let mut files = Vec::new();
        for i in 0..4 {
            let p = dir.path().join(format!("f{i}.txt"));
            std::fs::write(&p, format!("word {i} word {i}\n")).unwrap();
            files.push(p.to_str().unwrap().to_string());
        }
        let refs: Vec<&str> = files.iter().map(|s| s.as_str()).collect();

        let threaded = parallel_word_count_threads(&refs);
        let rt = tokio::runtime::Runtime::new().unwrap();
        let async_ = rt.block_on(parallel_word_count_async(&refs));

        assert_eq!(threaded, 16); // 4 files x 4 words
        assert_eq!(async_, 16);
    }
}
