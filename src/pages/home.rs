use crate::components::utils::TagButton;
use leptos::prelude::*;
#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="max-w-225 mx-auto">
            <div class="relative m-7">
                <div class="absolute right-0 bottom-0 size-[85%] bg-[#eae2dc] -z-100"></div>
                <div class="-z-10 mask-t-from-0% mask-b-to-100% bg-[url(/assets/images/luiza_bnner.png)] bg-contain bg-no-repeat">
                    <img
                        src="/assets/images/luiza_bnner.png"
                        alt="Luiza"
                        class="-z-10 max-w-[80%]"
                    />
                </div>
                <img
                    src="/assets/images/luiza_bnner.png"
                    alt="Luiza"
                    class="absolute top-0 left-0 max-w-[80%] opacity-80 brightness-110"
                />
                <div class="-translate-x-4 -translate-y-2 z-100 font-gistesy -rotate-8 atext md:text-8xl text-nowrap">
                    <h1 class="opacity-50 z-100 text-center">
                        "Wild hearts don’t follow rules."
                    </h1>
                    <h2 class="text-center z-100">"they rewrite them."</h2>
                </div>
            </div>
        </div>

        <div class="mb-5">
            <div class="grid grid-cols-1 grid-rows-1 place-items-center">
                <p class="z-10 px-3 py-3 text-xl text-center row-start-1 row-end-1 col-start-1 col-end-1 leading-9">
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
                <img
                    src="/assets/images/kobiety_przy_swiecy.png"
                    alt="Happy womans"
                    class="w-full h-auto row-start-1 row-end-1 col-start-1 col-end-1 opacity-20 object-fill"
                />
            </div>
        </div>
        <TagButton bname="1h MASTERCLASS" />
        <div class="bg-[url(/assets/images/taniec_kobieta.png)] bg-no-repeat bg-cover">
            <div class="bg-white/80 text-xl">
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
                <div class="flex basis-2/3">
                    <button class="bg-white rounded-3xl p-2">"Let's go WILD!"</button>
                </div>
            </div>
        </div>
    }
}
