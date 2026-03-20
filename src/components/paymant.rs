use leptos::prelude::*;

#[component]
pub fn CheckoutPage() -> impl IntoView {
    let (email, set_email) = signal("".to_string());

    let token = "test_03ea1e075f335d2b2f3cdf22b4c";
    let price_id = "pri_01kgw2nvx9mf3a3d7neqz9jpkn";
    let price_id2 = "pri_01kkmp3eedgkwphde7w3wrjvs7";

    let script = format!(
        r#"
Paddle.Environment.set('sandbox');
Paddle.Initialize({{ token: '{token}' }});
Paddle.Checkout.open({{
settings: {{
variant: 'one-page',
displayMode: 'inline',
frameTarget: 'paddle-checkout-box',
frameInitialHeight: 450,
frameStyle: 'width: 100%; background-color: transparent; border: none;'
}},
items: [{{ priceId: '{price_id}', quantity: 1 }}]
customer: {{email: '{:?email}'}},
}});

function toggleItem(checkbox) {{
if (checkbox.checked) {{
Paddle.Checkout.updateCheckout({{
items: [
{{ priceId: '{price_id}', quantity: 1 }},
{{ priceId: '{price_id2}', quantity: 1 }}
]
}});
}} else {{
Paddle.Checkout.updateCheckout({{
items: [{{ priceId: '{price_id}', quantity: 1 }}]
}});
}}
}}
"#
    );

    view! {
    <div>
    </div>

    <div class="sc-DWqcv kLCWFP min-h-[450]">

        <label id="check">
            <input type="checkbox" onclick="toggleItem(this)" />
            "Add coaching session"
        </label>
        <label for="name">
            <input type="email" on:input:target=move |ev| {set_email.set(ev.target().value());} />
        </label>
        <div class="paddle-checkout-box w-full min-h-80"></div>
        <script>{script}</script>
    </div>
    }
}
