use leptos::prelude::*;
#[component]
pub fn TagButton(
    /// Name of separator uppercase
    #[prop()]
    bname: &'static str
    ) -> impl IntoView {
    view! {
        <div class="flex justify-center translate-y-3">
            <h4 class="bg-[#f9f7f3] px-2 rounded-2xl">"↓ "{bname.to_string()}" ↓"</h4>
        </div>
    }
}
