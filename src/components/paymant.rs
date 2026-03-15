use leptos::prelude::*;

#[component]
pub fn CheckoutPage() -> impl IntoView {
    let token = "test_03ea1e075f335d2b2f3cdf22b4c";
    let price_id = "pri_01kgw2nvx9mf3a3d7neqz9jpkn";
    let price_id2 = "pri_01kkmp3eedgkwphde7w3wrjvs7";

    let script = format!(
        r#"
Paddle.Environment.set('sandbox');
Paddle.Initialize({{ token: '{}' }});
Paddle.Checkout.open({{
settings: {{
displayMode: 'inline',
frameTarget: 'paddle-checkout-box',
frameInitialHeight: 700,
frameStyle: 'width: 100%; background-color: transparent; border: none;'
}},
items: [{{ priceId: '{}', quantity: 1 }}]
}});

function toggleItem(checkbox) {{
if (checkbox.checked) {{
Paddle.Checkout.updateCheckout({{
items: [
{{ priceId: '{}', quantity: 1 }},
{{ priceId: '{}', quantity: 1 }}
]
}});
}} else {{
Paddle.Checkout.updateCheckout({{
items: [{{ priceId: '{}', quantity: 1 }}]
}});
}}
}}
"#,
        token, price_id, price_id, price_id2, price_id
    );

    view! {
    <div>
        <label>
            <input type="checkbox" onclick="toggleItem(this)" />
            "Add coaching session"
        </label>
    </div>
    <div class="flex flex-col items-center justify-start w-full mx-auto">
        <div class="paddle-checkout-box w-full min-h-80"></div>
        <script>{script}</script>
    </div>
    }
}
