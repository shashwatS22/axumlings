// tracing instrument
//
// `#[instrument]` automatically creates a tracing span for a function, logging
// all of its arguments and capturing any events inside it within that span's context.
//
// I AM NOT DONE

// TODO: import `tracing::instrument` and `tracing::info`

#[derive(Debug)]
struct User {
    id: u64,
    name: String,
}

// TODO: Add `#[instrument]` attribute to this function
async fn process_user(user: &User) {
    // TODO: log an info event with `tracing::info!` that says "processing user"
    // like this: `tracing::info!("processing user");`
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
