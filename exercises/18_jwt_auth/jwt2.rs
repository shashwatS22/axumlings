// Decode JWT
//
// Verify and decode a JWT, extracting the `Claims`.
//
// I AM NOT DONE

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub fn decode_token(token: &str, secret: &str) -> std::result::Result<Claims, ()> {
    // TODO: Use jsonwebtoken::decode with DecodingKey and Validation. Map errors to Err(())
    todo!()
}

fn main() {
    // Run `cargo test --bin jwt2` to test your solution!
}

#[cfg(test)]
mod tests {
    use super::*;
    use jsonwebtoken::{encode, EncodingKey, Header};

    #[test]
    fn test_decode() {
        let claims = Claims {
            sub: "alice".into(),
            exp: 10000000000,
        };
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(b"secret"),
        )
        .unwrap();

        let decoded = decode_token(&token, "secret");
        assert!(decoded.is_ok());
        assert_eq!(decoded.unwrap().sub, "alice");
    }
}
