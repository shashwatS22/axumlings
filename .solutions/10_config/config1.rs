// config Crate Layering
//
// Layer config file + env vars using the config crate. Deserialize into Settings.
// Create config/default.json and load with Config::builder().
//
// Hint: config::Config::builder().add_source(File::with_name("config").required(false)).add_source(Environment::default()).build()?.try_deserialize()

use axum::{routing::get, Json, Router};
use config::{Config, Environment, File};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Settings {
    host: String,
    port: u16,
}

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

async fn handler() -> Json<Settings> {
    let settings = Config::builder()
        .add_source(File::with_name("config/default").required(false))
        .add_source(Environment::with_prefix(""))
        .build()
        .unwrap()
        .try_deserialize::<Settings>()
        .unwrap_or_else(|_| Settings {
            host: "127.0.0.1".into(),
            port: 8080,
        });
    Json(settings)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_config() {
        std::env::set_var("PORT", "9000");
        let app = app();
        let response = app
            .oneshot(Request::builder().uri("/config").body(Body::empty()).unwrap())
            .await
            .unwrap();
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["port"], 9000);
        std::env::remove_var("PORT");
    }
}
