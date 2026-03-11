use std::sync::Arc;
use std::thread;

fn main() {
    let data = Arc::new(42);
    let data_clone = data.clone();
    let handle = thread::spawn(move || *data_clone);
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
