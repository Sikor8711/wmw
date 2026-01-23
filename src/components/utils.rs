use crate::models::CustomerData;
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
            <h4 class="bg-white px-2 rounded-2xl">"↓ "{bname.to_string()}" ↓"</h4>
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
                    <p class="text-xl">"Confirm email."</p>
                    <p>{data.first_name}"- please confirm your email addres"</p>
                    <p>"psst: Check spam folder and move to inbox"</p>
                    <p>"XOXO"</p>
                </div>
            }.into_any(),
            None => view! {
                <h2 class="text-[1.2rem] pb-3 text-center">"The Magnetic Message"</h2>
                <p class="text-[1rem] text-center">"A soulful guide to finding the message that your dream clients can feel — and can’t resist."</p>
                <form on:submit=on_submit class="mx-auto space-y-3 pt-3">
                    <input required class="border w-full p-1" type="text" placeholder="First name"
                        node_ref=first_name_ref
                    />
                    <input required class="border w-full p-1" type="email" placeholder="Email"
                        node_ref=email_ref
                    />
                    <button type="submint" class="w-full text-center text-black text-nowrap p-2 bg-(--bg-rose)">"DIVE INTO FREE GUIDE"</button>
                </form>
            }.into_any()
        }}
    }
}

#[component]
pub fn MauticTracker() -> impl IntoView {
    view! {
        <script>
            {r#"
                (function(w,d,t,u,n,a,m){w['MauticTrackingObject']=n;
                w[n]=w[n]||function(){(w[n].q=w[n].q||[]).push(arguments)},a=d.createElement(t),
                m=d.getElementsByTagName(t)[0];a.async=1;a.src=u;m.parentNode.insertBefore(a,m)
                })(window,document,'script','https://m.wildlymagnetic.co/mtc.js','mt');
                mt('send', 'pageview');
            "#}    
        </script>
    }
}
