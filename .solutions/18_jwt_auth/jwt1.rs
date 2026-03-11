// Encode JWT
//
// Create a signed JWT from a `Claims` struct. The `exp` (expiry) field
// is required for a valid token — always set it.
//

use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub fn create_token(secret: &str, subject: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let claims = Claims {
        sub: subject.to_string(),
        exp: 10000000000,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
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
