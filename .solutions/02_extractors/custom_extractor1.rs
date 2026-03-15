// Custom FromRequestParts
//
// Implement FromRequestParts for AuthUser to extract Bearer token from Authorization header.
// Any handler that needs auth adds AuthUser as a parameter.
//
// Hint: impl FromRequestParts for AuthUser, extract header, strip "Bearer " prefix

use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{header, request::Parts, StatusCode},
    routing::get,
    Router,
};

fn app() -> Router {
    Router::new().route("/me", get(handler))
}

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .unwrap();
    axum::serve(listener, app()).await.unwrap();
}

struct AuthUser {
    token: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let token = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.strip_prefix("Bearer ").map(|t| t.to_string()))
            .ok_or(StatusCode::UNAUTHORIZED)?;
        Ok(AuthUser { token })
    }
}

async fn handler(auth: AuthUser) -> String {
    format!("token: {}", auth.token)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_auth_ok() {
        let app = app();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/me")
                    .header("Authorization", "Bearer abc123")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        assert_eq!(&body[..], b"token: abc123");
    }

    #[tokio::test]
    async fn test_auth_unauthorized() {
        let app = app();
        let response = app
            .oneshot(Request::builder().uri("/me").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
}
