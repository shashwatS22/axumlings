// Full CRUD Routes
//
// Register all four CRUD verbs across two paths: /tasks (GET list, POST create)
// and /tasks/:id (PUT update, DELETE remove). Use method chaining.
//
// Hint: Two routes—one for /tasks (GET, POST), one for /tasks/:id (PUT, DELETE). Chain methods on each.

use axum::{extract::Path, routing::{delete, get, post, put}, Router};

fn app() -> Router {
    Router::new()
        .route("/tasks", get(list).post(create))
        .route("/tasks/:id", put(update).delete(remove))
}

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .unwrap();
    axum::serve(listener, app()).await.unwrap();
}

async fn list() -> &'static str {
    "[]"
}

async fn create(body: String) -> String {
    format!("created: {}", body)
}

async fn update(Path(id): Path<u32>, body: String) -> String {
    format!("updated {}: {}", id, body)
}

async fn remove(Path(id): Path<u32>) -> String {
    format!("deleted {}", id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Method, Request, StatusCode};
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_list() {
        let app = app();
        let response = app
            .oneshot(Request::builder().uri("/tasks").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        assert_eq!(&body[..], b"[]");
    }

    #[tokio::test]
    async fn test_create() {
        let app = app();
        let response = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri("/tasks")
                    .body(Body::from("task1"))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        assert_eq!(&body[..], b"created: task1");
    }

    #[tokio::test]
    async fn test_update() {
        let app = app();
        let response = app
            .oneshot(
                Request::builder()
                    .method(Method::PUT)
                    .uri("/tasks/42")
                    .body(Body::from("updated"))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        assert_eq!(&body[..], b"updated 42: updated");
    }

    #[tokio::test]
    async fn test_delete() {
        let app = app();
        let response = app
            .oneshot(
                Request::builder()
                    .method(Method::DELETE)
                    .uri("/tasks/99")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        assert_eq!(&body[..], b"deleted 99");
    }
}
