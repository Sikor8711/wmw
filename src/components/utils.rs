use crate::models::CustomerData;
use leptos::prelude::*;
use leptos::{ev::SubmitEvent, html::Input};
use std::env;
#[component]
pub fn TagButton(
    /// Name of separator uppercase
    #[prop()]
    bname: &'static str,
) -> impl IntoView {
    view! {
        <div class="flex justify-center translate-y-3">
            <h4 class="bg-white px-2 rounded-2xl">"↓ "{bname.to_string()}" ↓"</h4>
        </div>
    }
}
#[component]
pub fn NewsForm() -> impl IntoView {
    let (saved_data, set_saved_data) = signal::<Option<CustomerData>>(None);
    let first_name_ref: NodeRef<Input> = NodeRef::new();
    let email_ref: NodeRef<Input> = NodeRef::new();
    let add_contact = Action::new(|data: &CustomerData| {
        let data = data.clone();
        async move { add_mautic_contact(data).await }
    });
    let is_pending = add_contact.pending();
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let f_name = first_name_ref.get().expect("input").value();
        let email_val = email_ref.get().expect("input").value();
        let new_customer = CustomerData {
            first_name: f_name,
            email: email_val,
        };
        add_contact.dispatch(new_customer.clone());
        set_saved_data.set(Some(new_customer));
    };
    view! {
        {move || match saved_data.get() {
            Some(data) => view! {
                <div class="text-center mx-auto space-y-3 pt-3">
                    <p class="text-xl">"Confirm email."</p>
                    <p>{data.first_name}"- please confirm your email addres"</p>
                    <p>"psst: Check spam folder and move to inbox"</p>
                    <p>"XOXO"</p>
                </div>
            }.into_any(),
            None => view! {
                <h2 class="text-[1.2rem] pb-3 text-center">"The Magnetic Message"</h2>
                <p class="text-[1rem] text-center">"A soulful guide to finding the message that your dream clients can feel — and can’t resist."</p>
                <form on:submit=on_submit class="mx-auto space-y-3 pt-3">
                    <input required class="border w-full p-1" type="text" placeholder="First name"
                        node_ref=first_name_ref
                    />
                    <input required class="border w-full p-1" type="email" placeholder="Email"
                        node_ref=email_ref
                    />
                    <button type="submint" disabled=is_pending class="w-full text-center text-black text-nowrap p-2 bg-(--bg-rose)">
                        {move || if is_pending.get() {"SENDING..."}else{"DIVE INTO FREE GUIDE"}}
                    </button>
                </form>
            }.into_any()
        }}
    }
}
#[server(AddMuticContac, "/api")]
pub async fn add_mautic_contact(customer_data: CustomerData) -> Result<(), ServerFnError> {
    // 1. Logic inside this block only exists for the server
    #[cfg(feature = "ssr")]
    {
        use awc::http::{header, Method};
        use base64::{engine::general_purpose, Engine as _};
        use dotenvy::dotenv;
        use serde_json::json;
        use std::env;

        async fn matuic_api(
            method: Method,
            path: &str,
            body: Option<Value>,
        ) -> Result<Value, ServerFnError> {
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
        }
        let _ = dotenv();
        let url = env::var("MAUTIC_URL").expect("MAUTIC_URL missing");
        let login = env::var("MAUTIC_LOGIN").expect("MAUTIC_LOGIN missing");
        let password = env::var("MAUTIC_PASSWORD").expect("MAUTIC_PASSWORD missing");

        let cred = format!("{}:{}", login, password);
        let encode_cred = general_purpose::STANDARD.encode(cred.as_bytes());
        let auth = format!("Basic {}", encode_cred);

        let body = json!({
            "firstname": customer_data.first_name,
            "email": customer_data.email,
            "tags": ["Not-Confirm"]
        });

        let client = awc::Client::default();

        let mut check_email = client
            .get(format!(
                "{}/contacts?search=email:{}",
                url, customer_data.email
            ))
            .insert_header((header::AUTHORIZATION, auth.clone()))
            .send()
            .await
            .map_err(|e| {
                println!(">>> Request Failed: {:?}", e); // LOG 4
                ServerFnError::new(e.to_string())
            })?;

        if !check_email.status().is_success() {
            return Err(ServerFnError::new(format!(
                "Mautic error: {}",
                check_email.status()
            )));
        }

        let res = check_email.json::<serde_json::Value>().await?;
        let total = res["total"].as_str().unwrap_or("0");
        let email_id = res["contacts"]
            .as_object()
            .and_then(|map| map.keys().next());

        println!(
            "total email {}, email id {}",
            total,
            email_id.unwrap_or(&"Not Found".to_string())
        );

        let mut final_id: Option<String> = None;
        if let Some(id) = email_id {
            final_id = Some(id.to_string());
        } else {
            let mut new_contact = client
                .post(format!("{}/contacts/new", url))
                .insert_header((header::AUTHORIZATION, auth.clone()))
                .send_json(&body)
                .await
                .map_err(|e| {
                    println!(">>> Request Failed: {:?}", e); // LOG 4
                    ServerFnError::new(e.to_string())
                })?;

            if !new_contact.status().is_success() {
                return Err(ServerFnError::new(format!(
                    "Mautic error: {}",
                    new_contact.status()
                )));
            }

            println!(">>> New contact created! <<<"); // LOG 4
            let res = new_contact.json::<serde_json::Value>().await?;
            let new_contact_id = res["contact"]["id"].as_u64().map(|id| id.to_string());
            final_id = Some(new_contact_id.unwrap_or("No id".to_string()));
        }
        Ok(()) // Return for the SSR block
    }

    // 2. Logic inside this block only exists for the browser
    #[cfg(not(feature = "ssr"))]
    {
        // This is just a placeholder so the browser compiles cleanly.
        // It never actually runs because the server macro intercepts the call.
        let _ = customer_data;
        Ok(())
    };
}
