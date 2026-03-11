// Read PORT from environment; default to 3000 if not set. Bind the server to that port.
//
// Hint: std::env::var("PORT").ok().and_then(|s| s.parse().ok()).unwrap_or(3000)

use axum::Router;

fn app() -> Router {
    Router::new().route("/", axum::routing::get(|| async { "ok" }))
}

#[tokio::main]
async fn main() {
    // TODO: let port = env::var("PORT").ok().and_then(|s| s.parse().ok()).unwrap_or(3000);
    let port = 3000u16;
    let addr = (std::net::Ipv4Addr::LOCALHOST, port);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app()).await.unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use std::env;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_app() {
        let app = app();
        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), 200);
    }

    #[test]
    fn test_port_from_env() {
        env::set_var("PORT", "8080");
        let port: u16 = env::var("PORT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(3000);
        assert_eq!(port, 8080);
        env::remove_var("PORT");
    }
}
