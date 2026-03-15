// Broadcast Channel
//
// POST /broadcast sends a message to all subscribers via tokio::sync::broadcast.
// Store broadcast::Sender<String> in AppState; handler calls tx.send(msg).
//
// Hint: tokio::sync::broadcast::channel(32); tx.send(msg) is sync, returns Result

use axum::{extract::State, routing::post, Router};
use std::sync::Arc;
use tokio::sync::broadcast;

fn app() -> Router {
    let (tx, _rx) = broadcast::channel(32);
    Router::new()
        .route("/broadcast", post(handler))
        .with_state(Arc::new(tx))
}

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .unwrap();
    axum::serve(listener, app()).await.unwrap();
}

async fn handler(
    State(tx): State<Arc<broadcast::Sender<String>>>,
    body: String,
) -> &'static str {
    let _ = tx.send(body);
    "ok"
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_broadcast() {
        let (tx, mut rx) = broadcast::channel(32);
        let app = Router::new()
            .route("/broadcast", post(handler))
            .with_state(Arc::new(tx));
        let _ = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/broadcast")
                    .body(Body::from("event"))
                    .unwrap(),
            )
            .await
            .unwrap();
        let received = rx.recv().await.unwrap();
        assert_eq!(received, "event");
    }
}
