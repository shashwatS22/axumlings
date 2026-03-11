// Use sqlx SqlitePool to query a simple table. GET /users returns rows from users table.
//
// Hint: sqlx::SqlitePool::connect, pool.fetch_all(sqlx::query_as!(...))

use axum::{extract::State, routing::get, Router};
use sqlx::SqlitePool;

#[derive(Debug, serde::Serialize)]
struct User {
    id: i64,
    name: String,
}

fn app(pool: SqlitePool) -> Router {
    Router::new()
        .route("/users", get(handler))
        .with_state(pool)
}

#[tokio::main]
async fn main() {
    let pool = SqlitePool::connect("sqlite::memory:")
        .await
        .unwrap();
    sqlx::query("CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY, name TEXT)")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("INSERT INTO users (id, name) VALUES (1, 'Alice'), (2, 'Bob')")
        .execute(&pool)
        .await
        .unwrap();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .unwrap();
    axum::serve(listener, app(pool)).await.unwrap();
}

async fn handler(State(_pool): State<SqlitePool>) -> String {
    // TODO: Use pool.fetch_all with query_as to get User rows, serialize to JSON
    "[]".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_users() {
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        sqlx::query("CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT)")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("INSERT INTO users (id, name) VALUES (1, 'Alice'), (2, 'Bob')")
            .execute(&pool)
            .await
            .unwrap();

        let app = app(pool);
        let response = app
            .oneshot(Request::builder().uri("/users").body(Body::empty()).unwrap())
            .await
            .unwrap();
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert!(json.is_array());
        assert_eq!(json.as_array().unwrap().len(), 2);
    }
}
