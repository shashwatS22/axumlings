// Server-Sent Events
//
// Stream events to the client using SSE. GET /events returns an SSE stream
// that sends "tick" every 500ms. Use Sse::new(stream) with Event::default().data("tick").
//
// Hint: axum::response::sse::{Event, Sse}; tokio_stream::wrappers::IntervalStream

use axum::{routing::get, Router};
use std::time::Duration;

fn app() -> Router {
    Router::new().route("/events", get(handler))
}

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .unwrap();
    axum::serve(listener, app()).await.unwrap();
}

async fn handler() -> impl axum::response::IntoResponse {
    // TODO: Create stream: IntervalStream::new(interval(Duration::from_millis(500)))
    // TODO: .map(|_| Ok(Event::default().data("tick")))
    // TODO: Sse::new(stream)
    "not implemented".into_response()
}

use axum::response::IntoResponse;

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_sse() {
        let app = app();
        let response = app
            .oneshot(Request::builder().uri("/events").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.headers().get("content-type").and_then(|v| v.to_str().ok()), Some("text/event-stream"));
    }
}
