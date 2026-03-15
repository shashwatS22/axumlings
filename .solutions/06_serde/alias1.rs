// alias & rename
//
// Accept "database_url" as alias for db_url on input; output "maxConn" for max_conn.
// Use #[serde(alias = "database_url")] and #[serde(rename = "maxConn")].
//
// Hint: #[serde(alias = "database_url")] on db_url; #[serde(rename = "maxConn", alias = "max_conn")] on max_conn

use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct Config {
    #[serde(alias = "database_url")]
    db_url: String,
    max_conn: u32,
}

#[derive(Debug, Serialize)]
struct Response {
    db_url: String,
    #[serde(rename = "maxConn")]
    max_conn: u32,
}

fn app() -> Router {
    Router::new().route("/config", post(handler))
}

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .unwrap();
    axum::serve(listener, app()).await.unwrap();
}

async fn handler(Json(c): Json<Config>) -> Json<Response> {
    Json(Response {
        db_url: c.db_url,
        max_conn: c.max_conn,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_alias_database_url() {
        let app = app();
        let body = r#"{"database_url":"postgres:///db","max_conn":10}"#;
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/config")
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(json["db_url"], "postgres:///db");
        assert_eq!(json["max_conn"], 10);
    }

    #[tokio::test]
    async fn test_rename_output() {
        let app = app();
        let body = r#"{"db_url":"x","max_conn":5}"#;
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/config")
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(json["maxConn"], 5);
    }
}
