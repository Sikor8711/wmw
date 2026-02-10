#[cfg(feature = "ssr")]
pub mod ssr {
    // ✅ FIX 1: Import HeaderMap from 'http', not 'extract'
    use axum::{
        body::Bytes,
        http::{HeaderMap, StatusCode},
        response::IntoResponse,
    };
    use paddle_rust_sdk::{webhooks::MaximumVariance, Paddle};
    use std::env;

    pub async fn paddle_callback(headers: HeaderMap, body: String) -> impl IntoResponse {
        // 1. Extract Signature
        let maybe_signature = headers
            .get("paddle-signature")
            .and_then(|h| h.to_str().ok());

        let Some(signature) = maybe_signature else {
            return (StatusCode::BAD_REQUEST, "Missing Signature").into_response();
        };

        // 2. Get Secret
        let key = env::var("PADDLE_WEBHOOK_SECRET").expect("PADDLE_WEBHOOK_SECRET must be set");

        // 3. Verify & Parse
        match Paddle::unmarshal(&body, &key, signature, MaximumVariance::default()) {
            Ok(event) => {
                tokio::spawn(async move {
                    println!("Processing Event: {:?}", event);
                });
                (StatusCode::OK, "Webhook received").into_response()
            }
            Err(e) => {
                eprintln!("Webhook Verification Failed: {:?}", e);
                (StatusCode::BAD_REQUEST, "Invalid Signature").into_response()
            }
        }
    }
}
