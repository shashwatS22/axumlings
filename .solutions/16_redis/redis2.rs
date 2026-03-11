// Redis Pipeline
//
// Pipelining allows you to send multiple commands to Redis in a single network
// round-trip. This can significantly improve performance for bulk operations.
//
// In this exercise, complete the `run_pipeline` function.
//

use redis::AsyncCommands;

pub async fn run_pipeline(con: &mut deadpool_redis::Connection) -> redis::RedisResult<()> {
    // Create a pipeline using `redis::pipe()`
    // - Add a `.set("a", "1")`
    // - Add a `.set("b", "2")`
    // - Add a `.incr("counter", 1)`
    // - Execute it asynchronously using `.query_async(con).await` and return the result
    redis::pipe()
        .set("a", "1")
        .set("b", "2")
        .incr("counter", 1)
        .query_async(con)
        .await
}

fn main() {
    // This exercise relies on compile-time checks!
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_compiles() {
        assert!(true); // We just need the code to compile correctly!
    }
}
