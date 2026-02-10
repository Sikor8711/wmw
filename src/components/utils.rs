use crate::models::CustomerData;
use leptos::prelude::*;
use std::env;

#[cfg(feature = "ssr")]
pub async fn mautic_api(
    method: reqwest::Method, // CHANGED: awc::http::Method -> reqwest::Method
    path: &str,
    body: Option<serde_json::Value>,
) -> Result<serde_json::Value, ServerFnError> {
    use dotenvy::dotenv;
    use serde_json::Value;

    let _ = dotenv();
    let url = env::var("MAUTIC_URL").expect("MAUTIC_URL missing");
    let login = env::var("MAUTIC_LOGIN").expect("MAUTIC_LOGIN missing");
    let password = env::var("MAUTIC_PASSWORD").expect("MAUTIC_PASSWORD missing");

    let final_path = format!("{}/{}", url, path);

    // 1. Create Client (Note: In production, better to grab this from AppState)
    let client = reqwest::Client::new();

    // 2. Build Request
    let request_builder = client
        .request(method, final_path)
        .basic_auth(login, Some(password)); // CHANGED: Built-in Basic Auth

    // 3. Add Body if present
    let request_builder = if let Some(b) = body {
        request_builder.json(&b) // CHANGED: .json() instead of .send_json()
    } else {
        request_builder
    };

    // 4. Send and Handle Errors
    let response = request_builder
        .send()
        .await
        .map_err(|e| ServerFnError::new(format!("Network Error: {}", e)))?;

    // 5. Check HTTP Status (e.g. 404, 500)
    let response = response
        .error_for_status() // CHANGED: Reqwest helper to catch non-200 codes
        .map_err(|e| ServerFnError::new(format!("Mautic Error: {}", e)))?;

    // 6. Parse JSON
    let res_json: Value = response
        .json()
        .await
        .map_err(|e| ServerFnError::new(format!("Json Parse Error: {}", e)))?;

    Ok(res_json)
}

#[server(AddMuticContac, "/api")]
pub async fn add_mautic_contact(customer_data: CustomerData) -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use reqwest::Method; // CHANGED
        use serde_json::json;
        use std::env;
        use uuid::Uuid;

        let res = mautic_api(
            Method::GET,
            &format!("contacts?search=email:{}", customer_data.email),
            None,
        )
        .await?;

        // Extract ID safely
        let email_id = res["contacts"]
            .as_object()
            .and_then(|map| map.keys().next())
            .map(|k| k.to_string()); // Ensure we own the string

        let my_token = Uuid::new_v4().to_string();
        let body = json!({
            "firstname": customer_data.first_name,
            "email": customer_data.email,
            "tags": ["Not-Confirm"],
            "optintoken": my_token
        });

        if let Some(id) = email_id {
            println!("Email exists id: {}", id);
            // Optional: You might want to update the existing contact here
        } else {
            // Create new contact
            let res = mautic_api(Method::POST, "contacts/new", Some(body.clone())).await?;

            // Handle ID extraction which can be a number or string in JSON
            let new_id_option = res["contact"]["id"]
                .as_u64()
                .map(|id| id.to_string())
                .or_else(|| res["contact"]["id"].as_str().map(|s| s.to_string()));

            if let Some(new_id) = new_id_option {
                let segment_id = env::var("MAUTIC_SEGMENT_ID").expect("MAUTIC_SEGMENT_ID missing");
                let email_template_id = env::var("OPTIN_EMAIL_ID").expect("OPTIN_EMAIL_ID missing");

                // Add to Segment
                let _ = mautic_api(
                    Method::POST,
                    &format!("segments/{}/contact/{}/add", segment_id, new_id),
                    None,
                )
                .await?;

                // Send Email
                let _ = mautic_api(
                    Method::POST,
                    &format!("emails/{}/contact/{}/send", email_template_id, new_id),
                    None,
                )
                .await?;
            }
        }
    }
    Ok(())
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct VerificationResult {
    pub email: String,
    pub is_new_subscription: bool,
}

#[server(VerifyAndConfirm, "/api")]
pub async fn verify_and_confirm(token: String) -> Result<VerificationResult, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use reqwest::Method; // CHANGED
        use serde_json::json;

        let search_path = format!("contacts?search=optintoken:{}", token);
        let res = mautic_api(Method::GET, &search_path, None).await?;

        let (contact_id, email, is_unsubscribed) = res["contacts"]
            .as_object()
            .and_then(|map| {
                let id = map.keys().next()?;
                let email = map[id]["fields"]["core"]["email"]["value"].as_str()?;
                // Handle doNotContact list safely
                let is_unsubscribed = map[id]["doNotContact"]
                    .as_array()
                    .map(|arr| !arr.is_empty())
                    .unwrap_or(false);

                Some((id.clone(), email.to_string(), is_unsubscribed))
            })
            .ok_or_else(|| ServerFnError::new("Link Invalid or expired"))?;

        // Remove from Do Not Contact list if needed
        if is_unsubscribed {
            let _ = mautic_api(
                Method::POST,
                &format!("contacts/{}/dnc/email/remove", contact_id),
                None,
            )
            .await?;
        }

        // Update Contact Tags
        let update_body = json!({
            "tags": ["Confirmed", "-Not-Confirm"],
            "double_optin_confirm" : true, // Ensure this custom field alias is correct in Mautic
        });

        let _ = mautic_api(
            Method::PATCH,
            &format!("contacts/{}/edit", contact_id),
            Some(update_body),
        )
        .await?;

        // Move Segments
        let confirmed_segment =
            std::env::var("CONFIRMED_SEGMENT_ID").unwrap_or_else(|_| "4".to_string());
        let _ = mautic_api(
            Method::POST,
            &format!("segments/{}/contact/{}/add", confirmed_segment, contact_id),
            None,
        )
        .await?;

        let remove_segment = std::env::var("MAUTIC_SEGMENT_ID").unwrap_or_else(|_| "3".to_string());
        let _ = mautic_api(
            Method::POST,
            &format!("segments/{}/contact/{}/remove", remove_segment, contact_id),
            None,
        )
        .await?;

        Ok(VerificationResult {
            email: email.to_string(),
            is_new_subscription: !is_unsubscribed,
        })
    }
    #[cfg(not(feature = "ssr"))]
    {
        let _ = token;
        // Just a stub for the compiler on client side
        Err(ServerFnError::new("Function not available on client"))
    }
}
