use chrono::prelude::*;
use leptos::prelude::*;
#[component]
pub fn Footer() -> impl IntoView {
    let dt = Utc::now();
    view! {
        <div class="p-3 bg-[#8a7769] text-sm text-white">
            <div class="grid grid-cols-1 sm:grid-cols-2 place-items-center ">
                <div>
                    <p class="pr-1 font-gistesy text-[1rem]">" Wildly Magnetic"</p>
                </div>
                <div>
                    <p class="">
                        "START YOUR TRANSFORMATION HERE ↓"
                    </p>
                    <p>
                        "Get wildly magnetic Instagram strategies + empowering"
                    </p>
                    <p>
                        "business inspiration delivered to your inbox every week."
                    </p>
                </div>



            </div>
            <div class="py-2 flex  justify-center text-[0.5rem]">
                <p class="pr-1">{dt.year()}" "</p>
                <p class="pr-1 font-gistesy text-[0.6rem]">" Wildly Magnetic"</p>
                <p>"All right reserve"</p>
            </div>
        </div>
    }
}
