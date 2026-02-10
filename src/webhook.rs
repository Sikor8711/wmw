use actix_web::{post, HttpRequest, HttpResponse, Responder};
use paddle_rust_sdk::{webhooks::MaximumVariance, Paddle};
use std::env;

#[post("/paddle-callback")]
pub async fn paddle_callback(request_body: String, req: HttpRequest) -> impl Responder {
    let maybe_signature = req
        .headers()
        .get("paddle-signature")
        .and_then(|h| h.to_str().ok());

    let Some(signature) = maybe_signature else {
        return HttpResponse::BadRequest().body("Missing Signature");
    };
    let key = env::var("PADDLE_WEBHOOK_SECRET").expect("PADDLE_WEBHOOK_SECRET must be set");

    match Paddle::unmarshal(&request_body, &key, signature, MaximumVariance::default()) {
        Ok(event) => {
            actix_web::rt::spawn(async move {
                println!("Processing Event: {:?}", event);
            });
            HttpResponse::Ok().body("Webhook recieved")
        }
        Err(e) => {
            eprintln!("Webhook Verification Vaild: {:?}", e);
            HttpResponse::BadRequest().body("Invalid Signature")
        }
    }
}
