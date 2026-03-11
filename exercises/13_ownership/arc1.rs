// Use Arc to share ownership across threads. Clone the Arc and pass to a thread.
//
// Hint: Arc::new(data), arc.clone() for sharing

use std::sync::Arc;
use std::thread;

fn main() {
    let data = Arc::new(42);
    // TODO: Clone data and move into thread
    let handle = thread::spawn(|| {
        // Use the shared data
        0
    });
    let _ = handle.join();
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_arc_share() {
        let data = Arc::new(42);
        let data_clone = data.clone();
        let handle = thread::spawn(move || *data_clone);
        assert_eq!(handle.join().unwrap(), 42);
    }
}
