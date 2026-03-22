use crate::components::lemon::LemonPayment;
use leptos::prelude::*;
#[component]
pub fn AboutPage() -> impl IntoView {
    let price_data = Resource::new(|| (), |_| async move { get_price().await });
    view! {
    <div class="max-w-225 mx-auto">
        <p>"About Page"</p>
        <p>"hej"</p>
        <p>"hej hej"</p>
    </div>
    <Suspense fallback=move || view! { <p>"Fetching price from server..."</p> }>
        {move || match price_data.get() {
        Some(Ok(price)) => view! {
        <p>"Live Price: "{price}</p>
        }.into_any(),
        Some(Err(e)) => view! {
        <p>{e.to_string()}</p>
        }.into_any(),
        None => view! { <p>"Wait for it"</p> }.into_any(),
        }}
    </Suspense>

    <LemonPayment />

    }
}

#[server(PaddlePrice, "/api")]
pub async fn get_price() -> Result<String, ServerFnError> {
    use paddle_rust_sdk::Paddle;
    let _client = Paddle::new(
        std::env::var("PADDLE_API_KEY").unwrap_or_default(),
        Paddle::SANDBOX,
    )?;
    Ok("23.20".to_string())
}
