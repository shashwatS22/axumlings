// Lua Script via EVALSHA
//
// Run Lua scripts atomically on the Redis server. Scripts have access to KEYS and ARGV
// arrays, can call any Redis command, and execute without interruption from other clients.
//
// In this exercise, complete the `run_script` function.
//
// I AM NOT DONE

use redis::AsyncCommands;

pub async fn run_script(
    con: &mut deadpool_redis::Connection,
    key: &str,
    amount: i64,
) -> redis::RedisResult<i64> {
    let lua_src = r#"
        return redis.call('INCRBY', KEYS[1], ARGV[1])
    "#;

    // TODO: Create a script using `redis::Script::new(lua_src)`
    // TODO: Invoke it asynchronously with `.invoke_async(con).await`
    // Hint: `.key(key).arg(amount)` chains before `.invoke_async` in modern `redis` crates,
    // or use whatever `Script` API is available!
    // Actually, the Script pattern is:
    // redis::Script::new(lua_src).key(key).arg(amount).invoke_async(con).await

    todo!()
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
