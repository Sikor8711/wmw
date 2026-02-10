use serde::{Deserialize, Serialize};

// --- SHARED DATA STRUCTURES ---
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PaddleItem {
    pub price_id: String,
    pub quantity: u32,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionRequest {
    pub items: Vec<PaddleItem>,
}

#[derive(Deserialize)]
pub struct TransactionResponse {
    pub data: TransactionData,
}

#[derive(Deserialize)]
pub struct TransactionData {
    pub id: String,
}

// --- SERVER ONLY CODE ---
#[cfg(feature = "ssr")]
pub mod ssr {
    use super::*;
    use crate::state::AppState;
    use axum::{extract::State, http::StatusCode, response::IntoResponse, Json}; // Correct import

    pub async fn create_payment(
        State(state): State<AppState>,
        Json(price_id): Json<String>,
    ) -> impl IntoResponse {
        let payload = TransactionRequest {
            items: vec![PaddleItem {
                price_id,
                quantity: 1,
            }],
        };

        let client = reqwest::Client::new();

        let response = client
            .post("https://sandbox-api.paddle.com/transactions")
            .bearer_auth(&state.paddle_key)
            .json(&payload)
            .send()
            .await;

        match response {
            Ok(res) => {
                if res.status().is_success() {
                    match res.json::<TransactionResponse>().await {
                        Ok(body) => (StatusCode::OK, Json(body.data.id)).into_response(),
                        Err(e) => {
                            println!("JSON Parse Error: {}", e);
                            (StatusCode::INTERNAL_SERVER_ERROR, "Invalid Upstream JSON")
                                .into_response()
                        }
                    }
                } else {
                    let error_text = res.text().await.unwrap_or_default();
                    println!("Paddle API Error: {}", error_text);
                    (StatusCode::INTERNAL_SERVER_ERROR, "Paddle API Error").into_response()
                }
            }
            Err(e) => {
                println!("Network Request Failed: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Network Error").into_response()
            }
        }
    }
}
