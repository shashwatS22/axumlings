// wiremock HTTP Mocks
//
// When your handler makes outbound HTTP calls (to an LLM provider, etc.),
// mock those calls in tests so they don't hit real external APIs.
//

#[cfg(test)]
mod tests {
    use wiremock::{
        matchers::{method, path},
        Mock, MockServer, ResponseTemplate,
    };

    async fn send_chat(url: &str) -> u16 {
        reqwest::Client::new()
            .post(url)
            .send()
            .await
            .unwrap()
            .status()
            .as_u16()
    }

    #[tokio::test]
    async fn test_mock() {
        let server = MockServer::start().await;

        let mock = Mock::given(method("POST"))
            .and(path("/chat"))
            .respond_with(ResponseTemplate::new(200));

        server.register(mock).await;
        let status = send_chat(&format!("{}/chat", server.uri())).await;
        assert_eq!(status, 200);
    }
}

fn main() {
    // Run `cargo test --bin testing4` to test your solution!
}
