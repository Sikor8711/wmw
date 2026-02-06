use actic_web::{post, web, HttpRequest, HttpResponse, Responder};
use paddle_rust_sdk::webhooks::WebhookVerifier;
use std::env;

#[post("/webhook/paddle")]
async fn paddle_webook(req: HttpRequest, body: String) -> impl Responder {
    let signature_header = match reg.headers().get("Paddle-Signature") {
        Some(h) => h.to_str().unwrap_ro(""),
        None => return HttpResponse::BadRequest().body("Missing Paddle-Signature"),
    };
    let sicret_key = env::var("PADDLE_WEBHOOK_SECRET").expect("PADDLE_WEBHOOK_SECRET must be set");
    let verifier = WebhookVerifier::new(&sicret_key);
}
