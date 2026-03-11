// Get, Set, and Del
//
// In this exercise, complete the `cache_user` and `get_user` functions.
// You don't need a running Redis server to pass this exercise—it just needs to compile!
//
// I AM NOT DONE

use redis::AsyncCommands;

pub async fn cache_user(
    con: &mut deadpool_redis::Connection,
    user_id: u32,
    data: &str,
) -> redis::RedisResult<()> {
    // TODO: use `.set_ex` to store `data` under the key `format!("user:{}", user_id)` for 60 seconds
    todo!()
}

pub async fn get_user(
    con: &mut deadpool_redis::Connection,
    user_id: u32,
) -> redis::RedisResult<Option<String>> {
    // TODO: use `.get` to retrieve the key `format!("user:{}", user_id)`
    todo!()
}

pub async fn clear_user(
    con: &mut deadpool_redis::Connection,
    user_id: u32,
) -> redis::RedisResult<()> {
    // TODO: use `.del` to remove the key `format!("user:{}", user_id)`
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
