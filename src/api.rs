use actix_web::{get, post, web, HttpResponse, Responder};
use awc::Client;
use serde::{Deserialize, Serialize};

pub struct AppState {
    pub paddle_client: Client,
    pub paddle_api_key: String,
}

#[derive(Serialize)]
struct PaddleItem {
    price_id: String,
    quantity: u32,
}

#[derive(Serialize)]
struct TransactionRequest {
    items: Vec<PaddleItem>,
}
#[derive(Deserialize)]
struct TransactionResponse {
    data: TransactionData,
}

#[derive(Deserialize)]
struct TransactionData {
    id: String,
}

#[post("/api/create-payment")]
pub async fn create_payment(
    state: web::Data<AppState>,
    price_id: web::Json<String>,
) -> impl Responder {
    let payload = TransactionRequest {
        items: vec![PaddleItem {
            price_id: price_id.into_inner(),
            quantity: 1,
        }],
    };
    let response = state
        .paddle_client
        .post("https://sandbox-api.paddle.com/transactions")
        .bearer_auth(&state.paddle_api_key)
        .send_json(&payload)
        .await;
    match response {
        Ok(mut res) => {
            if res.status().is_success() {
                let body: TransactionResponse = res.json().await.unwrap();
                HttpResponse::Ok().json(body.data.id)
            } else {
                let error_body = res.body().await.unwrap();
                println!("Paddle Error: {:?}", error_body);
                HttpResponse::InternalServerError().body("Paddle API Error")
            }
        }
        Err(e) => {
            println!("Request faild: {}", e);
            HttpResponse::InternalServerError().body("Network Error")
        }
    }
}
