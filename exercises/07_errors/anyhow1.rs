// anyhow Context
//
// Use .context("message") to add context when propagating errors.
// Refactor the handler to use anyhow::Context instead of manual map_err.
//
// Hint: .context("failed to read config") instead of .map_err(|e| anyhow::anyhow!(e))

use axum::{routing::get, Router};
use anyhow::{Context, Result};

fn app() -> Router {
    Router::new().route("/config", get(handler))
}

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .unwrap();
    axum::serve(listener, app()).await.unwrap();
}

fn load_port() -> Result<u16> {
    let s = std::env::var("PORT")
        .map_err(|e| anyhow::anyhow!(e))?;  // TODO: Use .context("failed to read PORT") instead
    Ok(s.parse::<u16>()
        .map_err(|e| anyhow::anyhow!(e))?)  // TODO: Use .context("PORT must be a number") instead
}

async fn handler() -> Result<String, String> {
    let port = load_port().map_err(|e| e.to_string())?;
    Ok(format!("port: {}", port))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_context() {
        std::env::set_var("PORT", "8080");
        let app = app();
        let response = app
            .oneshot(Request::builder().uri("/config").body(Body::empty()).unwrap())
            .await
            .unwrap();
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        assert_eq!(&body[..], b"port: 8080");
        std::env::remove_var("PORT");
    }
}
