use chrono::prelude::*;
use leptos::prelude::*;
#[component]
pub fn Footer() -> impl IntoView {
    let dt = Utc::now();
    view! {

        <div class="bg-[#8a7769]">
            <div class="py-2 flex text-white justify-center text-[0.5rem]">
                <p class="pr-1">{dt.year()}" "</p>
                <p class="pr-1 font-gistesy text-[0.6rem]">" Wildly Magnetic"</p>
                <p>"All right reserve"</p>
            </div>
        </div>
    }
}
