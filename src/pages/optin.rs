use crate::components::utils::verify_and_confirm;
use leptos::prelude::*;
use leptos_router::hooks::use_query_map;

#[component]
pub fn OptIn() -> impl IntoView {
    let query = use_query_map();
    let token = move || query.with(|m| m.get("optintoken").unwrap_or_default());

    let send_token = Action::new(|token: &String| {
        let token = token.clone();
        async move { verify_and_confirm(token).await }
    });

    let result = send_token.value();
    let is_pending = send_token.pending();

    view! {
        <div class="w-full h-[70vh] flex justify-center items-center bg-[url(/assets/images/luiza_bnner8.webp)] bg-no-repeat bg-top bg-cover">
            <div class="animate-soulful text-center space-y-5 bg-white/70 p-8 rounded-2xl">
            {move || match result.get()
                {
                    Some(Ok(data)) => {
                        if data.is_new_subscription {
                            view! {
                                <div class="animate-soulful">
                                    <h1 class="text-xl md:text-3xl text-green-700">"Success"</h1>
                                    <p class="text-md md:text-xl">"Thank you. "{data.email} " is now verified."</p>
                                    <p class="pt-4 text-sm">"You can now close this window."</p>
                                </div>
                            }.into_any()
                        } else {
                            view! {
                                <div class="animate-soulful">
                                    <h1 class="text-xl md:text-3xl text-green-700">"Success"</h1>
                                    <p class="text-md md:text-xl">"Welcome back!. "{data.email} " is now verified."</p>
                                    <p class="pt-4 text-sm">"You can now close this window."</p>
                                </div>
                            }.into_any()
                        }
                    }.into_any(),
                    Some(Err(e)) => view! {
                        <div class="animate-soulful">
                            <h1 class="text-xl md:text-3xl text-red-600">"Verification Failed"</h1>
                            <p>{e.to_string()}</p>
                            <button
                            on:click=move |_| {send_token.dispatch(token());}
                            class="mt-4 text-sm underline"
                            >
                            "Try again"
                            </button>
                        </div>
                    }.into_any(),
                    None => view! {
                        <h1 class="animate-soulful text-xl md:text-3xl">"Please confirm your email address"</h1>
                            <a href=move || format!("/optin?optintoken={}&submited=true", token())>
                                <button
                                on:click=move |_| {send_token.dispatch(token());}
                                disabled=is_pending
                                class="animate-soulful text-nowrap text-black text-md md:text-xl bg-(--bg-rose) h-auto rounded-full p-4 shadow-2xl hover:scale-105 transition-transform disabled:opacity-50"
                                >
                                    {move || if is_pending.get() { "Confirming..." } else { "Confirm" }}
                                </button>
                            </a>
                    }.into_any(),
                }}
            </div>
        </div>
    }
}
