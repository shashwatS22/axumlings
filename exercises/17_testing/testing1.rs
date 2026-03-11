// #[tokio::test]
//
// Write async unit tests. `#[test]` doesn't work for async functions
// you need `#[tokio::test]` which spins up a Tokio runtime for the duration of the test.
//
// Complete the test to verify `add(2, 3)` equals `5`.
//
// I AM NOT DONE

pub async fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    // TODO: use super::*;
    // TODO: write async test function `test_add` using `#[tokio::test]`
}

fn main() {
    // Run `cargo test --bin testing1` to test your solution!
}
