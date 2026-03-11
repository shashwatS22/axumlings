// tracing instrument
//
// `#[instrument]` automatically creates a tracing span for a function, logging
// all of its arguments and capturing any events inside it within that span's context.
//

use tracing::{info, instrument};

#[derive(Debug)]
struct User {
    id: u64,
    name: String,
}

#[instrument]
async fn process_user(user: &User) {
    info!("processing user");
}

fn main() {
    // Run `cargo test --bin obs1` to test your solution!
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_compiles() {
        assert!(true);
    }
}
