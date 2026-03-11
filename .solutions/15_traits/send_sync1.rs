use std::sync::Arc;
use std::thread;

fn send_to_thread<T: Send + 'static>(t: T) {
    thread::spawn(move || {
        let _ = t;
    });
}

fn main() {
    let x = Arc::new(42);
    send_to_thread(x);
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use super::send_to_thread;

    #[test]
    fn test_send() {
        let x = Arc::new(42);
        send_to_thread(x);
    }
}
