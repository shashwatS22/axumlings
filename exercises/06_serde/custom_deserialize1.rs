// Custom Deserialize
//
// Implement Deserialize for Color to parse hex strings like "#ff8800" into RGB.
// Use a Visitor with visit_str to parse the hex bytes.
//
// Hint: impl Deserialize for Color with struct V; impl Visitor for V { fn visit_str -> parse hex }

use axum::{routing::post, Json, Router};
use serde::{de::Visitor, Deserialize, Deserializer, Serialize};

#[derive(Debug, Serialize)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ColorVisitor;

        impl<'de> Visitor<'de> for ColorVisitor {
            type Value = Color;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a hex color string like #rrggbb")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let s = v.strip_prefix('#').unwrap_or(v);
                if s.len() != 6 {
                    return Err(E::custom("hex color must be 6 characters"));
                }
                let r = u8::from_str_radix(&s[0..2], 16)
                    .map_err(|_| E::custom("invalid hex"))?;
                let g = u8::from_str_radix(&s[2..4], 16)
                    .map_err(|_| E::custom("invalid hex"))?;
                let b = u8::from_str_radix(&s[4..6], 16)
                    .map_err(|_| E::custom("invalid hex"))?;
                Ok(Color { r, g, b })
            }
        }

        deserializer.deserialize_str(ColorVisitor)
    }
}

fn app() -> Router {
    Router::new().route("/color", post(handler))
}

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .unwrap();
    axum::serve(listener, app()).await.unwrap();
}

#[derive(Deserialize)]
struct Payload {
    color: Color,
}

async fn handler(Json(p): Json<Payload>) -> Json<Color> {
    Json(p.color)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_custom_deserialize() {
        let app = app();
        let body = r##"{"color":"#ff8800"}"##;
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/color")
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(json["r"], 255);
        assert_eq!(json["g"], 136);
        assert_eq!(json["b"], 0);
    }
}
