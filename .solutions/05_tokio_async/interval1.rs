// Interval Timer
//
// Spawn a background task that increments a counter every 100ms using tokio::time::interval.
// GET /count returns the current tick count.
//
// Hint: let mut tick = interval(Duration::from_millis(100)); loop { tick.tick().await; count.fetch_add(1, Ordering::SeqCst); }

use axum::{extract::State, routing::get, Router};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::interval;

fn app() -> Router {
    let count = Arc::new(AtomicUsize::new(0));
    let count_clone = count.clone();
    tokio::spawn(async move {
        let mut tick = interval(Duration::from_millis(100));
        loop {
            tick.tick().await;
            count_clone.fetch_add(1, Ordering::SeqCst);
        }
    });
    Router::new()
        .route("/count", get(handler))
        .with_state(count)
}

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .unwrap();
    axum::serve(listener, app()).await.unwrap();
}

async fn handler(State(count): State<Arc<AtomicUsize>>) -> String {
    format!("{}", count.load(Ordering::SeqCst))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_interval() {
        let count = Arc::new(AtomicUsize::new(0));
        let count_clone = count.clone();
        tokio::spawn(async move {
            let mut tick = interval(Duration::from_millis(10));
            for _ in 0..3 {
                tick.tick().await;
                count_clone.fetch_add(1, Ordering::SeqCst);
            }
        });
        tokio::time::sleep(Duration::from_millis(50)).await;
        let app = Router::new()
            .route("/count", get(handler))
            .with_state(count);
        let response = app
            .oneshot(Request::builder().uri("/count").body(Body::empty()).unwrap())
            .await
            .unwrap();
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let n: usize = std::str::from_utf8(&body).unwrap().parse().unwrap();
        assert!(n >= 2, "expected at least 2 ticks, got {}", n);
    }
}
