// POST /send sends a message on a channel; the handler receives the Sender from state.
// Use tokio::sync::mpsc - state: Arc<mpsc::Sender<String>>.
//
// Hint: tokio::sync::mpsc::channel(32); sender.send(msg).await

use axum::{extract::State, routing::post, Router};
use std::sync::Arc;
use tokio::sync::mpsc;

fn app() -> Router {
    let (tx, _rx) = mpsc::channel(32);
    Router::new()
        .route("/send", post(handler))
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
    State(_tx): State<Arc<mpsc::Sender<String>>>,
    body: String,
) -> &'static str {
    // TODO: _tx.send(body).await.ok(); return "ok"
    "ok"
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_mpsc() {
        let (tx, mut rx) = mpsc::channel(32);
        let app = Router::new()
            .route("/send", post(handler))
            .with_state(Arc::new(tx));
        let _ = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/send")
                    .body(Body::from("hello"))
                    .unwrap(),
            )
            .await
            .unwrap();
        let received = rx.recv().await.unwrap();
        assert_eq!(received, "hello");
    }
}
