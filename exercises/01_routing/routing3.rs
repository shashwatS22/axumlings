// Add nested routes: GET /api/users and GET /api/users/:id
// Use Router::nest() for the /api prefix and path extraction for :id.
//
// Hint: Use Router::nest() for sub-routes.

use axum::{extract::Path, routing::get, Router};

fn app() -> Router {
    let app = Router::new();
    // TODO: Nest under "/api" a router with get("/users", list_users) and get("/users/:id", get_user)
    app
}

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .unwrap();
    axum::serve(listener, app()).await.unwrap();
}

async fn list_users() -> &'static str {
    "[]"
}

async fn get_user(Path(id): Path<u32>) -> String {
    format!("user {}", id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_list_users() {
        let app = app();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/users")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        assert_eq!(&body[..], b"[]");
    }

    #[tokio::test]
    async fn test_get_user() {
        let app = app();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/users/42")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        assert_eq!(&body[..], b"user 42");
    }
}
