// Encode JWT
//
// Create a signed JWT from a `Claims` struct. The `exp` (expiry) field
// is required for a valid token — always set it.
//
// I AM NOT DONE

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    // TODO: add `exp: usize` field
}

pub fn create_token(secret: &str, subject: &str) -> std::result::Result<String, ()> {
    // TODO: use jsonwebtoken::encode, Header, and EncodingKey
    // Hint: `jsonwebtoken::encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes()))`
    todo!()
}

fn main() {
    // Run `cargo test --bin jwt1` to test your solution!
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_encode() {
        assert!(create_token("secret", "user123").is_ok());
    }
}
