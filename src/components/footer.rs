use chrono::prelude::*;
use leptos::prelude::*;
#[component]
pub fn Footer() -> impl IntoView {
    let dt = Utc::now();
    view! {
        <div class="bg-(--bg-footer) text-sm text-white py-5 px-[10%]">
            <div class="grid gap-5 grid-cols-1 sm:grid-cols-2 place-items-center text-center">
                <div>
                    <p class="pr-1 font-gistesy text-lg">"Wildly Magnetic"</p>
                </div>
                <div class="space-y-2">
                    <p>
                        "START YOUR TRANSFORMATION HERE ↓"
                    </p>
                    <p class="text-justify">
                        "Get wildly magnetic Instagram strategies + empowering business inspiration delivered to your inbox every week."
                    </p>
                </div>
            </div>
            <div class="pt-5 flex  justify-center text-[0.6rem]">
                <p class="pr-1">{dt.year()}" "</p>
                <p class="pr-1 font-gistesy text-[0.6rem]">" Wildly Magnetic"</p>
                <p>"All right reserve"</p>
            </div>
        </div>
    }
}
