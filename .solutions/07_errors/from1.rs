// #[from] Conversions
//
// Add #[from] to error variants that wrap other error types so ? works across them.
// ConfigError::Io(#[from] io::Error) and ConfigError::Parse(#[from] ParseIntError).
//
// Hint: Io(#[from] std::io::Error), Parse(#[from] std::num::ParseIntError)

use axum::{routing::get, Router};
use thiserror::Error;

#[derive(Debug, Error)]
enum ConfigError {
    #[error("IO error")]
    Io(#[from] std::io::Error),
    #[error("Parse error")]
    Parse(#[from] std::num::ParseIntError),
}

fn app() -> Router {
    Router::new().route("/parse", get(handler))
}

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .unwrap();
    axum::serve(listener, app()).await.unwrap();
}

fn parse_port(s: &str) -> Result<u16, ConfigError> {
    // With #[from], ? will convert io::Error and ParseIntError to ConfigError
    let _ = std::fs::read_to_string(s)?;  // io::Error -> ConfigError::Io
    Ok("42".parse::<u16>()?)               // ParseIntError -> ConfigError::Parse
}

async fn handler() -> Result<String, String> {
    let port = parse_port("/nonexistent").map_err(|e| e.to_string())?;
    Ok(format!("port: {}", port))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_from_conversion() {
        let app = app();
        let response = app
            .oneshot(Request::builder().uri("/parse").body(Body::empty()).unwrap())
            .await
            .unwrap();
        // Should fail with Io error (file not found) - 500 with error message
        assert!(response.status().is_server_error() || response.status().is_client_error());
    }
}
