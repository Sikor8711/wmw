use crate::models::CustomerData;
use leptos::prelude::*;
use std::env;

#[cfg(feature = "ssr")]
pub async fn mautic_api(
    method: awc::http::Method,
    path: &str,
    body: Option<serde_json::Value>,
) -> Result<serde_json::Value, ServerFnError> {
    use awc::http::header;
    use base64::{engine::general_purpose, Engine as _};
    use dotenvy::dotenv;
    use serde_json::Value;

    let _ = dotenv();
    let url = env::var("MAUTIC_URL").expect("MAUTIC_URL missing");
    let login = env::var("MAUTIC_LOGIN").expect("MAUTIC_LOGIN missing");
    let password = env::var("MAUTIC_PASSWORD").expect("MAUTIC_PASSWORD missing");

    let cred = format!("{}:{}", login, password);
    let encode_cred = general_purpose::STANDARD.encode(cred.as_bytes());
    let auth = format!("Basic {}", encode_cred);
    let final_path = format!("{}/{}", url, path);
    let client = awc::Client::default();
    let mut response = if let Some(b) = body {
        client
            .request(method, final_path)
            .insert_header((header::AUTHORIZATION, auth))
            .send_json(&b)
            .await
    } else {
        client
            .request(method, final_path)
            .insert_header((header::AUTHORIZATION, auth))
            .send()
            .await
    }
    .map_err(|e| ServerFnError::new(e.to_string()))?;
    let res_json: Value = response
        .json()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(res_json)
}

#[server(AddMuticContac, "/api")]
pub async fn add_mautic_contact(customer_data: CustomerData) -> Result<(), ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        use awc::http::Method;
        use serde_json::json;
        use std::env;
        use uuid::Uuid;

        let res = mautic_api(
            Method::GET,
            &format!("contacts?search=email:{}", customer_data.email),
            None,
        )
        .await?;

        let email_id = res["contacts"]
            .as_object()
            .and_then(|map| map.keys().next());

        let my_token = Uuid::new_v4().to_string();
        let body = json!({
            "firstname": customer_data.first_name,
            "email": customer_data.email,
            "tags": ["Not-Confirm"],
            "optintoken": my_token
        });

        if let Some(id) = email_id {
            println!("Email exists id: {}", id);
        } else {
            //create new contact
            let res = mautic_api(Method::POST, "contacts/new", Some(body.clone())).await?;
            if let Some(new_id) = res["contact"]["id"]
                .as_u64()
                .map(|id| id.to_string())
                .or_else(|| res["contact"]["id"].as_str().map(|s| s.to_string()))
            {
                let segment_id = env::var("MAUTIC_SEGMENT_ID").expect("MAUTIC_SEGMENT_ID missing");
                let email_id = env::var("OPTIN_EMAIL_ID").expect("OPTIN_EMAIL_ID missing");

                // const sendConfirmationEmail = await fetch(`${MAUTIC_BASE_URL}/emails/${MAUTIC_COMFIRMATION_EMAIL_ID}/contact/${contactId}/send`, {
                let _ = mautic_api(
                    Method::POST,
                    &format!("segments/{}/contact/{}/add", segment_id, new_id),
                    None,
                )
                .await?;
                let _ = mautic_api(
                    Method::POST,
                    &format!("emails/{}/contact/{}/send", email_id, new_id),
                    None,
                )
                .await?;
            }
        }
        Ok(()) // Return for the SSR block
    }
    #[cfg(not(feature = "ssr"))]
    {
        let _ = customer_data;
        Ok(())
    };
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
        use awc::http::Method;
        use serde_json::json;
        let search_path = format!("contacts?search=optintoken:{}", token);
        let res = mautic_api(Method::GET, &search_path, None).await?;
        let (contact_id, email, is_unsubscribed) = res["contacts"]
            .as_object()
            .and_then(|map| {
                let id = map.keys().next()?;
                let email = map[id]["fields"]["core"]["email"]["value"].as_str()?;
                let dnc_list = map[id]["doNotContact"].as_array()?;
                let is_unsubscribed = !dnc_list.is_empty();
                Some((id.clone(), email.to_string(), is_unsubscribed))
            })
            .ok_or_else(|| ServerFnError::new("Link Invalid or expired"))?;

        if is_unsubscribed {
            let _ = mautic_api(
                Method::POST,
                &format!("contacts/{}/dnc/email/remove", contact_id),
                None,
            )
            .await?;
        }

        let update_body = json!({
            "tags": ["Confirmed","-Not-Confirm"],
            "double_optin_confirm" : true,
        });
        let _ = mautic_api(
            Method::PATCH,
            &format!("contacts/{}/edit", contact_id),
            Some(update_body),
        )
        .await?;
        let confirmed_segment = std::env::var("CONFIRMED_SEGMENT_ID").unwrap_or("4".to_string());
        let _ = mautic_api(
            Method::POST,
            &format!("segments/{}/contact/{}/add", confirmed_segment, contact_id),
            None,
        )
        .await?;
        let remove_segment = std::env::var("MAUTIC_SEGMENT_ID").unwrap_or("3".to_string());

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
        Err(ServerFnError::new("Function not aveilable on client"))
    }
}
