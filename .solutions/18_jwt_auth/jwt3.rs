// Bearer Token Extractor
//
// Combine header extraction and JWT decoding in a `FromRequestParts` impl.
// Any handler adds `AuthUser` as a parameter and gets a validated `claims` struct.
//

use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{header::AUTHORIZATION, request::Parts, StatusCode},
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub struct AuthUser {
    pub claims: Claims,
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for AuthUser {
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .ok_or(StatusCode::UNAUTHORIZED)?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or(StatusCode::UNAUTHORIZED)?;

        let decoded = decode::<Claims>(
            token,
            &DecodingKey::from_secret(b"secret"),
            &Validation::default(),
        )
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

        Ok(AuthUser {
            claims: decoded.claims,
        })
    }
}

fn main() {
    // Run `cargo test --bin jwt3` to test your solution!
}

#[cfg(test)]
mod tests {
    use super::*;
    use http::Request;
    use jsonwebtoken::{encode, EncodingKey, Header};

    #[tokio::test]
    async fn test_auth() {
        let claims = Claims {
            sub: "bob".into(),
            exp: 10000000000,
        };
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(b"secret"),
        )
        .unwrap();

        let mut req = Request::builder()
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .body(())
            .unwrap();

        let (mut parts, _) = req.into_parts();
        let user = AuthUser::from_request_parts(&mut parts, &()).await.unwrap();
        assert_eq!(user.claims.sub, "bob");
    }
}
