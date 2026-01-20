use chrono::prelude::*;
use leptos::prelude::*;
#[component]
pub fn Footer() -> impl IntoView {
    let dt = Utc::now();
    view! {
        <div class="bg-(--bg-darker) text-sm text-white py-5 px-[10%]">
            <div class="grid gap-5 grid-cols-1 sm:grid-cols-2 place-items-center text-center">
                <div>
                    <p class="pr-1 font-gistesy text-lg">"Wildly Magnetic"</p>
                </div>
                <div class="space-y-2">
                    <p class="text-[1rem] pb-5">
                        "START YOUR TRANSFORMATION HERE ↓"
                    </p>
                    <p class="text-justify pb-2">
                        "Get wildly magnetic Instagram strategies + empowering business inspiration delivered to your inbox every week."
                    </p>
                    <form class="relative w-full flex justify-end">
                        <img class="absolute bottom-[-20] sm:bottom-[-10] left-0" src="assets/images/heart_line.avif" alt="line with heart"/>
                        <input class="w-[70%] rounded-lg text-center p-1" placeholder="sign up for newsletter" type="email"/>
                    </form>
                </div>
            </div>
            <div class="pt-5 pb-2 flex justify-center gap-3 text-[0.8rem]">
                <a href="/terms-of-use">"Terms of use"</a>
                <a href="/privacy-policy">"Privacy Policy"</a>
            </div>
            <div class="flex  justify-center text-[0.8rem]">
                <p class="pr-1">{dt.year()}" "</p>
                <p class="pr-1 font-gistesy text-[0.8rem]">" Wildly Magnetic"</p>
                <p>"All right reserve"</p>
            </div>
        </div>
    }
}
