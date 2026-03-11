// Redis Pipeline
//
// Pipelining allows you to send multiple commands to Redis in a single network
// round-trip. This can significantly improve performance for bulk operations.
//
// In this exercise, complete the `run_pipeline` function.
//
// I AM NOT DONE

use redis::AsyncCommands;

pub async fn run_pipeline(con: &mut deadpool_redis::Connection) -> redis::RedisResult<()> {
    // TODO: Create a pipeline using `redis::pipe()`
    // - Add a `.set("a", "1")`
    // - Add a `.set("b", "2")`
    // - Add a `.incr("counter", 1)`
    // - Execute it asynchronously using `.query_async(con).await` and return the result
    todo!()
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
