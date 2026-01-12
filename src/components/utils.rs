use crate::models::CustomerData;
use leptos::logging::log;
use leptos::prelude::*;
use leptos::{ev::SubmitEvent, html::Input};
#[component]
pub fn TagButton(
    /// Name of separator uppercase
    #[prop()]
    bname: &'static str,
) -> impl IntoView {
    view! {
        <div class="flex justify-center translate-y-3">
            <h4 class="bg-[#f9f7f3] px-2 rounded-2xl">"↓ "{bname.to_string()}" ↓"</h4>
        </div>
    }
}
#[component]
pub fn NewsForm() -> impl IntoView {
    let (saved_data, set_saved_data) = signal::<Option<CustomerData>>(None);
    let first_name_ref: NodeRef<Input> = NodeRef::new();
    let email_ref: NodeRef<Input> = NodeRef::new();
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let f_name = first_name_ref.get().expect("input").value();
        let email_val = email_ref.get().expect("input").value();
        let new_customer = CustomerData {
            first_name: f_name,
            email: email_val,
        };
        set_saved_data.set(Some(new_customer));
    };
    view! {
        {move || match saved_data.get() {
            Some(data) => view! {
                <div class="text-center mx-auto space-y-3 pt-3">
                    <h3 class="text-3xl">"Thank you!"</h3>
                    <p>"Welcome, "{data.first_name}</p>
                    <p>"We will email you at: "{data.email}</p>

                </div>
            }.into_any(),
            None => view! {
                <form on:submit=on_submit class="text-xs mx-auto space-y-3 pt-3">
                    <input class="border w-full p-1" type="text" placeholder="First name"
                        node_ref=first_name_ref
                    />
                    <input class="border w-full p-1" type="email" placeholder="Email"
                        node_ref=email_ref
                    />
                    <button type="submint" class="w-full text-center text-xs text-[#554b43] text-nowrap p-2 bg-[#d9b8ae]">"DIVE INTO FREE GUIDE"</button>
                </form>
            }.into_any()
        }}
    }
}
