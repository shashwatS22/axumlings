// query_as! Macro
//
// Use sqlx::query_as! to map rows into a typed struct with compile-time SQL verification.
// GET /users returns rows from users table as JSON.
//
// Hint: sqlx::query_as!(User, "SELECT id, name FROM users").fetch_all(pool).await

use axum::{extract::State, routing::get, Json, Router};
use serde::Serialize;
use sqlx::{FromRow, SqlitePool};

#[derive(Debug, FromRow, Serialize)]
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

async fn handler(State(pool): State<SqlitePool>) -> Json<Vec<User>> {
    let rows = sqlx::query_as::<_, User>("SELECT id, name FROM users")
        .fetch_all(&pool)
        .await
        .unwrap();
    Json(rows)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_query_as() {
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
        assert_eq!(json[0]["name"], "Alice");
    }
}
