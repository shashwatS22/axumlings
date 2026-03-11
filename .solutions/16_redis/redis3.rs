// Lua Script via EVALSHA
//
// Run Lua scripts atomically on the Redis server. Scripts have access to KEYS and ARGV
// arrays, can call any Redis command, and execute without interruption from other clients.
//
// In this exercise, complete the `run_script` function.
//

use redis::AsyncCommands;

pub async fn run_script(
    con: &mut deadpool_redis::Connection,
    key: &str,
    amount: i64,
) -> redis::RedisResult<i64> {
    let lua_src = r#"
        return redis.call('INCRBY', KEYS[1], ARGV[1])
    "#;

    redis::Script::new(lua_src)
        .key(key)
        .arg(amount)
        .invoke_async(con)
        .await
}

fn main() {
    // This exercise relies on compile-time checks!
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_compiles() {
        assert!(true);
    }
}
