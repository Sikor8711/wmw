use crate::components::animatein::AnimateIn;
use crate::components::utils::add_mautic_contact;
use crate::models::CustomerData;
use leptos::prelude::*;
use leptos::{ev::SubmitEvent, html::Input};
#[component]
pub fn HeroSection() -> impl IntoView {
    view! {
        <div class="max-w-225 mx-auto">
            <div class="relative m-7">
                <div class="absolute right-0 bottom-0 size-[85%] bg-(--bg-sec) -z-100"></div>
                <div class="-z-10 mask-t-from-0% mask-b-to-100% bg-[url(/assets/images/luiza_bnner.webp)] bg-contain bg-no-repeat">
                    <picture>
                        <source srcset="/assets/images/luiza_bnner.webp" />
                    <img
                        src="/assets/images/luiza_bnner.webp"
                        width="800"
                        height="600"
                        fetchpriority="high"
                        loading="eager"
                        alt="Luiza"
                        decoding="async"
                        class="-z-10 max-w-[80%]"
                    />
                    </picture>
                </div>
                <picture>
                    <source srcset="/assets/images/luiza_bnner.webp" />
                    <img
                        src="/assets/images/luiza_bnner.webp"
                        width="800"
                        height="600"
                        fetchpriority="high"
                        loading="eager"
                        alt="Luiza"
                        decoding="async"
                        class="absolute top-0 left-0 max-w-[80%] opacity-80 brightness-110"
                    />
                </picture>
                <div class="-translate-x-4 -translate-y-2 z-100 font-gistesy -rotate-8 atext md:text-8xl text-nowrap">
                    <AnimateIn>
                    <h1 class="opacity-50 z-100 text-center">
                        "Wild hearts don’t follow rules."
                    </h1>
                    </AnimateIn>
                    <AnimateIn>
                    <h2 class="text-center z-100">"they rewrite them."</h2>
                    </AnimateIn>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn ManSection() -> impl IntoView {
    view! {
        <div class="mb-5">
            <div class="grid grid-cols-1 grid-rows-1 place-items-center">
                <img
                    src="/assets/images/kobiety_przy_swiecy.webp"
                    alt="Happy womans"
                    loading="lazy"
                    class="w-full h-auto row-start-1 row-end-1 col-start-1 col-end-1 opacity-20 object-fill"
                />
                <div class="z-10 row-start-1 row-end-1 col-start-1 col-end-1">
                    <p class="px-3 py-3 text-[5vw]/[8vw] md:text-4xl/14 text-center">
                        <b>"The place "</b>
                        "where wild-hearted"
                        <br />
                        "creators "
                        <b>"transform "</b>
                        "their "
                        <br />
                        "magic into power and their "
                        <br />
                        <b>"brand into legacy."</b>
                    </p>
                </div>
            </div>
        </div>

    }
}
#[component]
pub fn UnlockSection() -> impl IntoView {
    view! {
        <div class="bg-[url(/assets/images/taniec_kobieta.webp)] bg-no-repeat bg-cover">
            <div class="bg-white/80 text-xl">
                <AnimateIn>
                <div class="text-right pt-5 pr-2">
                    <p class="pt-3 mr-12">
                        <span class="font-gistesy text-5xl">"Unlock "</span>
                        "your wild magic &"
                    </p>
                    <p>"watch your business explode"</p>
                    <p class="mr-12">
                        "back to " <span class="font-gistesy text-5xl">"life."</span>
                    </p>
                </div>
                </AnimateIn>
                <AnimateIn>
                <div class="grid grid-cols-2 content-center mx-2">
                    <div class="place-self-center">
                    <p class="text-xs text-center bg-white rounded-3xl m-1">"1h MASTERCLASS"</p>
                        <img class="h-32" src="/assets/images/monitorek.webp" loading="lazy" alt=""/>
                    </div>
                    <div class="place-self-center">
                        <div class="h-full flex items-center">
                            <img class="max-w-8 mr-0.5" src="/assets/images/button_deco.webp" loading="lazy" alt=""/>
                            <button class="text-nowrap text-[1rem] bg-white h-auto rounded-3xl p-2 shadow-2xl ">"Let's go WILD!"</button>
                            <img class="scale-x-[-1] max-w-8 ml-0.5" src="/assets/images/button_deco.webp" loading="lazy" alt=""/>
                        </div>
                    </div>
                </div>
                </AnimateIn>
            </div>
        </div>
    }
}
#[component]
pub fn StopperSection() -> impl IntoView {
    view! {
            <AnimateIn>
            <div>
                <p class="py-8 text-center font-gistesy md:text-5xl text-[8vw] text-nowrap">"You’re freaking magnetic and you know it."</p>
            </div>
            </AnimateIn>
    }
}

// #[component]
// pub fn NewsForm() -> impl IntoView {
//     let (saved_data, set_saved_data) = signal::<Option<CustomerData>>(None);
//     let first_name_ref: NodeRef<Input> = NodeRef::new();
//     let email_ref: NodeRef<Input> = NodeRef::new();
//     let add_contact = Action::new(|data: &CustomerData| {
//         let data = data.clone();
//         async move { add_mautic_contact(data).await }
//     });
//     let is_pending = add_contact.pending();
//     let on_submit = move |ev: SubmitEvent| {
//         ev.prevent_default();
//         let f_name = first_name_ref.get().expect("input").value();
//         let email_val = email_ref.get().expect("input").value();
//         let new_customer = CustomerData {
//             first_name: f_name,
//             email: email_val,
//         };
//         add_contact.dispatch(new_customer.clone());
//         set_saved_data.set(Some(new_customer));
//     };
//     view! {
//         <div class="bg-[url(/assets/images/pokoj.webp)] bg-center bg-no-repeat bg-cover ">
//             <div class="py-10 bg-white/80">
//                 <div class="place-self-center pl-2 max-w-75">
//                     {move || match saved_data.get()
//                         {
//                             Some(data) => view! {
//                                 <div class="animate-soulful text-center mx-auto space-y-3 pt-3">
//                                     <p class="text-xl">"Confirm email."</p>
//                                     <p>{data.first_name}"- please confirm your email addres"</p>
//                                     <p>"psst: Check spam folder and move to inbox"</p>
//                                     <p>"XOXO"</p>
//                                 </div>
//                             }.into_any(),
//                             None => view! {
//                                 <AnimateIn>
//                                     <h2 class="text-[1.2rem] pb-3 text-center">"The Magnetic Message"</h2>
//                                 </AnimateIn>
//                                 <AnimateIn>
//                                     <p class="text-[1rem] text-center">"A soulful guide to finding the message that your dream clients can feel — and can’t resist."</p>
//                                 </AnimateIn>
//                                     <form on:submit=on_submit class="mx-auto space-y-3 pt-3">
//                                         <AnimateIn>
//                                             <input required class="border w-full p-1" type="text" placeholder="First name" node_ref=first_name_ref />
//                                         </AnimateIn>
//                                         <AnimateIn>
//                                             <input required class="border w-full p-1" type="email" placeholder="Email" node_ref=email_ref />
//                                         </AnimateIn>
//                                         <AnimateIn>
//                                             <button type="submit" disabled=is_pending class="w-full text-center text-black text-nowrap p-2 bg-(--bg-rose)">
//                                                 {move || if is_pending.get() {"SENDING..."}else{"DIVE INTO FREE GUIDE"}}
//                                             </button>
//                                         </AnimateIn>
//                                     </form>
//                             }.into_any()
//                         }}
//                 </div>
//             </div>
//         </div>
//     }
// }
#[component]
pub fn NewsForm() -> impl IntoView {
    let (saved_data, set_saved_data) = signal::<Option<CustomerData>>(None);
    let first_name_ref: NodeRef<Input> = NodeRef::new();
    let email_ref: NodeRef<Input> = NodeRef::new();
    let add_contact = Action::new(|data: &CustomerData| {
        let data = data.clone();
        async move { add_mautic_contact(data).await }
    });
    let is_pending = add_contact.pending();
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let f_name = first_name_ref.get().expect("input").value();
        let email_val = email_ref.get().expect("input").value();
        let new_customer = CustomerData {
            first_name: f_name,
            email: email_val,
        };
        add_contact.dispatch(new_customer.clone());
        set_saved_data.set(Some(new_customer));
    };
    view! {
        <div class="bg-[url(/assets/images/pokoj.webp)] bg-center bg-no-repeat bg-cover ">
            <div class="py-10 bg-white/80">
                <div class="place-self-center pl-2 max-w-75">
                    {move || match saved_data.get()
                        {
                            Some(data) => view! {<SuccessState name=data.first_name /> }.into_any(),
                            None => view! {
                                <FormState
                                    on_submit=on_submit
                                    is_pending=is_pending.into()
                                    f_ref=first_name_ref
                                    e_ref=email_ref
                                />
                            }.into_any()
                        }}
                </div>
            </div>
        </div>
    }
}
#[component]
fn SuccessState(name: String) -> impl IntoView {
    view! {
        <div class="animate-soulful text-center mx-auto space-y-3 pt-3">
            <p class="text-xl">"Confirm email."</p>
            <p>{name}"- please confirm your email addres"</p>
            <p>"psst: Check spam folder and move to inbox"</p>
            <p>"XOXO"</p>
        </div>
    }
}
#[component]
fn FormState(
    on_submit: impl Fn(SubmitEvent) + 'static + Clone,
    is_pending: Signal<bool>,
    f_ref: NodeRef<Input>,
    e_ref: NodeRef<Input>,
) -> impl IntoView {
    view! {
        <AnimateIn><h2 class="text-[1.2rem] pb-3 text-center">"The Magnetic Message"</h2></AnimateIn>
        <AnimateIn>
            <p class="text-[1rem] text-center">"A soulful guide to finding the message that your dream clients can feel."</p>
        </AnimateIn>

        <form on:submit=move |ev| on_submit(ev) class="mx-auto space-y-3 pt-3">
            <AnimateIn>
                <input required class="border w-full p-1" type="text" placeholder="First name" node_ref=f_ref />
            </AnimateIn>
            <AnimateIn>
                <input required class="border w-full p-1" type="email" placeholder="Email" node_ref=e_ref />
            </AnimateIn>
            <AnimateIn>
                <button type="submit" disabled=is_pending class="w-full text-center text-black p-2 bg-(--bg-rose)">
                    {move || if is_pending.get() { "SENDING..." } else { "DIVE INTO FREE GUIDE" }}
                </button>
            </AnimateIn>
        </form>
    }
}

#[component]
pub fn Stopper2() -> impl IntoView {
    view! {
        <TagButton bname="FREE GUIDE" />
        <div class="pt-8">
            <div class="h-8 bg-(--bg-sec)"> </div>
        </div>
    }
}
#[component]
pub fn TagButton(
    /// Name of separator uppercase
    #[prop()]
    bname: &'static str,
) -> impl IntoView {
    view! {
        <div class="flex justify-center translate-y-3 z-100">
            <h4 class="bg-white px-2 rounded-2xl">"↓ "{bname.to_string()}" ↓"</h4>
        </div>
    }
}
#[component]
pub fn MeetLuiza() -> impl IntoView {
    view! {
        <div class="space-y-10 bg-[url(/assets/images/liscie_o2.webp)] bg-center py-10">
        <AnimateIn>
            <h3 class="text-[5vw] md:text-3xl text-center">"Meet Luiza - the soul behind the"
            <br/>
            <span class="font-gistesy text-[7vw] md:text-4xl text-nowrap">"Wildly Magnetic"</span></h3>
        </AnimateIn>
            <img class="w-[65%] mx-auto" src="/assets/images/luiza_w_marynarce3.webp" loading="lazy" alt=""/>
        <AnimateIn>
            <p class="px-[10%] text-justify">
                "A woman who didn’t find herself in perfection, but in the mess of real life — in exhaustion, in starting over, in choosing herself again and again. She carries the depth, yes, but also the earth — the late nights, the quiet rebuild, the courage to walk away from a life that drained her."
            </p>
        </AnimateIn>
        <AnimateIn>
            <p class="px-[10%] text-justify">
                "Her work is where soul meets truth, guiding you back to the version of yourself your body remembers, your heart misses, and your future is already calling in."
            </p>
        </AnimateIn>
            <div class="mx-[10%] bg-(--bg-sec) text-[4vw]/[6vw] md:text-2xl text-center space-y-6 py-6">
                <AnimateIn>
                <p>
                    "You weren’t born to follow rules."
                    <br/>
                    "You were born to lead with soul."
                </p>
                </AnimateIn>
                <AnimateIn>
                <p class="font-gistesy text-[6vw] md:text-4xl">"Let’s get you there."</p>
                </AnimateIn>
            </div>

            <AnimateIn>
            <div class="flex justify-center">
                <button class="text-nowrap text-black text-xl bg-(--bg-rose) h-auto rounded-full p-4 shadow-2xl ">"Activate My Wild Potential!"</button>
            </div>
            </AnimateIn>
        </div>
    }
}
#[component]
pub fn WhatIDo() -> impl IntoView {
    view! {
        <div  class="relative bg-(--bg-darker) px-[10%] pb-10">
            <div class="relative text-sm text-(--bg-lighter) pt-10 space-y-6 z-50">
        <AnimateIn>
                <h3 class="text-2xl text-center pb-3">"The Heartfire Behind What I Do"</h3>
        </AnimateIn>
        <AnimateIn>
                <p class="text-justify z-50">" You were never meant to live half-alive. You were never meant to silence your magic to fit into a box the world handed you. "</p>
        </AnimateIn>
        <AnimateIn>
                <p class="text-justify">" Your soul has been calling for more — more freedom, more expression, more truth, more fire, more you. "</p>
        </AnimateIn>
        <AnimateIn>
                <p class="text-justify">"You were born to create from your core. To move with intention. To lead with energy. To be so wildly magnetic that your presence alone shifts the room."</p>
        </AnimateIn>
        <AnimateIn>
                <p class="text-justify">"And here…we don’t tame that. We amplify it. We build businesses that feel like breath, like warmth in your chest, like coming home after years of being away."</p>
        </AnimateIn>
        <AnimateIn>
                <p class="text-justify">
                    "This isn’t about chasing success."<br/>
                    "It’s about aligning with what was always yours."
                </p>
        </AnimateIn>
        <AnimateIn>
                <p class="text-justify">" If you’re ready to stop performing and start expressing… If you’re ready to trade force for flow, doubt for devotion, pressure for power… If you’re ready to build a brand that feels like freedom in your bones and expansion in your chest..."</p>
        </AnimateIn>
        <AnimateIn>
                <p class="text-justify">
                    "You’re in the right place."<br/>
                    "This is your space."
                </p>
        </AnimateIn>
        <AnimateIn>
                <p class="text-justify">" Welcome home."</p>
        </AnimateIn>
            </div>
            <img class="absolute  w-auto h-[60%] bottom-0 right-0 z-0" src="/assets/images/kobieta_i_swieca_2_3_luiza.webp" loading="lazy" alt="Woman and candle"/>
        </div>
    }
}
#[component]
pub fn FireInYou() -> impl IntoView {
    view! {
        <div class="bg-(--bg-sec)">
            <div class="space-y-6 py-10 px-[10%] bg-[url(/assets/images/liscie_o2.webp)] bg-center bg-size-[200%] bg-no-repeat">
                <AnimateIn>
                <p class="font-gistesy text-[7vw] md:text-4xl">"There’s a fire in you..."</p>
                </AnimateIn>
                <AnimateIn>
                <p class="font-gistesy text-[7vw] md:text-4xl text-center">"Let’s turn it into pure wild magnetism."</p>
                </AnimateIn>
                <AnimateIn>
                <div class="flex justify-center">
                    <button class="text-nowrap text-black text-xl bg-(--bg-rose) h-auto rounded-full p-4 shadow-2xl ">"I'm So Ready to Rise!"</button>
                </div>
                </AnimateIn>
            </div>
        </div>
    }
}
