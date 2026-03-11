use axum::{extract::Path, extract::State, routing::get, routing::post, Router};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

fn app() -> Router {
    let state = Arc::new(RwLock::new(HashMap::new()));
    Router::new()
        .route("/kv/:key", get(handler_get))
        .route("/kv/:key", post(handler_post))
        .with_state(state)
}

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .unwrap();
    axum::serve(listener, app()).await.unwrap();
}

async fn handler_get(
    State(store): State<Arc<RwLock<HashMap<String, String>>>>,
    Path(key): Path<String>,
) -> String {
    store
        .read()
        .unwrap()
        .get(&key)
        .cloned()
        .unwrap_or_else(|| "not found".to_string())
}

async fn handler_post(
    State(store): State<Arc<RwLock<HashMap<String, String>>>>,
    Path(key): Path<String>,
    body: String,
) -> &'static str {
    store.write().unwrap().insert(key, body);
    "ok"
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_kv() {
        let app = app();
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/kv/foo")
                    .body(Body::from("bar"))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), 200);
        let response = app
            .oneshot(Request::builder().uri("/kv/foo").body(Body::empty()).unwrap())
            .await
            .unwrap();
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        assert_eq!(&body[..], b"bar");
    }
}
