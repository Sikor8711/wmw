use leptos::prelude::*;
#[component]
pub fn OptIn() -> impl IntoView {
    let email = "example@example.com";
    view! {
        <div class="w-full h-[70vh] flex justify-center items-center bg-[url(/assets/images/luiza_bnner8.webp)] bg-no-repeat bg-top bg-cover">
            <div class="text-center space-y-5 bg-white/70 p-8 rounded-2xl">
                <h1 class="text-xl md:text-3xl">"Plseas confirm your email address"</h1>
                <p class="text-md md:text-xl">{email}</p>
                <button class="text-nowrap text-black text-md md:text-xl bg-(--bg-rose) h-auto rounded-full p-4 shadow-2xl ">"Confirm"</button>
            </div>

        </div>

    }
}
