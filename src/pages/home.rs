use crate::components::utils::NewsForm;
use crate::components::utils::TagButton;
use leptos::prelude::*;
#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="max-w-225 mx-auto">
            <div class="relative m-7">
                <div class="absolute right-0 bottom-0 size-[85%] bg-(--bg-sec) -z-100"></div>
                <div class="-z-10 mask-t-from-0% mask-b-to-100% bg-[url(/assets/images/luiza_bnner.webp)] bg-contain bg-no-repeat">
                    <picture>
                        <source src="/assets/images/luiza_bnner.webp" />
                        <source src="/assets/images/luiza_bnner.avif" />
                    <img
                        src="/assets/images/luiza_bnner.webp"
                        widht="800"
                        height="600"
                        fetchprority="high"
                        alt="Luiza"
                        decoding="async"
                        class="-z-10 max-w-[80%]"
                    />
                    </picture>
                </div>
                <picture>
                    <source src="/assets/images/luiza_bnner.webp" />
                    <source src="/assets/images/luiza_bnner.avif" />
                    <img
                        src="/assets/images/luiza_bnner.webp"
                        widht="800"
                        height="600"
                        fetchprority="high"
                        alt="Luiza"
                        decoding="async"
                        class="absolute top-0 left-0 max-w-[80%] opacity-80 brightness-110"
                    />
                </picture>
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
                <p class="z-10 px-3 py-3 text-[5vw]/[8vw] md:text-4xl/14 text-center row-start-1 row-end-1 col-start-1 col-end-1">
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
                    src="/assets/images/kobiety_przy_swiecy.webp"
                    alt="Happy womans"
                    loading="lazy"
                    class="w-full h-auto row-start-1 row-end-1 col-start-1 col-end-1 opacity-20 object-fill"
                />
            </div>
        </div>
        <TagButton bname="1h MASTERCLASS" />
        <div class="bg-[url(/assets/images/taniec_kobieta.webp)] bg-no-repeat bg-cover">
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
            </div>
        </div>
        <div>
            <p class="py-8 text-center font-gistesy md:text-5xl text-[8vw] text-nowrap">"You’re freaking magnetic and you know it."</p>
        </div>
        <TagButton bname="FREE GUIDE" />
        <div class="bg-[url(/assets/images/pokoj.webp)] bg-center bg-no-repeat bg-cover ">
        <div class="grid grid-cols-2 py-10 bg-white/80">
                <div class="place-self-center pl-2 w-[90%]">
                        <NewsForm/>
                </div>
                <div class="pt-3 pr-2">
                    <img src="/assets/images/monitorki.webp" loading="lazy" alt=""/>
                </div>
            </div>
        </div>
        <div class="pt-8">
            <div class="h-8 bg-(--bg-sec)"> </div>
        </div>
        <div class="space-y-10 bg-[url(/assets/images/liscie_o2.webp)] bg-center py-10">
            <h3 class="text-[5vw] md:text-3xl text-center">"Meet Luiza - the soul behind the"
            <br/>
            <span class="font-gistesy text-[7vw] md:text-4xl text-nowrap">"Wildly Magnetic"</span></h3>
            <img class="w-[65%] mx-auto" src="/assets/images/luiza_w_marynarce3.webp" loading="lazy" alt=""/>
            <p class="px-[10%] text-justify">
                "A woman who didn’t find herself in perfection, but in the mess of real life — in exhaustion, in starting over, in choosing herself again and again. She carries the depth, yes, but also the earth — the late nights, the quiet rebuild, the courage to walk away from a life that drained her."
            </p>
            <p class="px-[10%] text-justify">
                "Her work is where soul meets truth, guiding you back to the version of yourself your body remembers, your heart misses, and your future is already calling in."
            </p>
            <div class="mx-[10%] bg-(--bg-sec) text-[4vw]/[6vw] md:text-2xl text-center space-y-6 py-6">
                <p class="">
                    "You weren’t born to follow rules."
                    <br/>
                    "You were born to lead with soul."
                </p>
                <p class="font-gistesy text-[6vw] md:text-4xl">"Let’s get you there."</p>
            </div>
            <div class="flex justify-center">
                <button class="text-nowrap text-(--bg-darker) text-xl bg-(--bg-rose) h-auto rounded-full p-4 shadow-2xl ">"Activate My Wild Potential!"</button>
            </div>

        </div>
        <div  class="relative bg-(--bg-darker) px-[10%] pb-10">
            <div class="relative text-sm text-(--bg-lighter) pt-10 space-y-6 z-50">
                <h3 class="text-2xl text-center pb-3">"The Heartfire Behind What I Do"</h3>
                <p class="text-justify z-50">" You were never meant to live half-alive. You were never meant to silence your magic to fit into a box the world handed you. "</p>
                <p class="text-justify">" Your soul has been calling for more — more freedom, more expression, more truth, more fire, more you. "</p>
                <p class="text-justify">"You were born to create from your core. To move with intention. To lead with energy. To be so wildly magnetic that your presence alone shifts the room."</p>
                <p class="text-justify">"And here…we don’t tame that. We amplify it. We build businesses that feel like breath, like warmth in your chest, like coming home after years of being away."</p>
                <p class="text-justify">
                    "This isn’t about chasing success."<br/>
                    "It’s about aligning with what was always yours."
                </p>
                <p class="text-justify">" If you’re ready to stop performing and start expressing… If you’re ready to trade force for flow, doubt for devotion, pressure for power… If you’re ready to build a brand that feels like freedom in your bones and expansion in your chest..."</p>
                <p class="text-justify">
                    "You’re in the right place."<br/>
                    "This is your space."
                </p>
                <p class="text-justify">" Welcome home."</p>
            </div>
            <img class="absolute  w-auto h-[60%] bottom-0 right-0 z-0" src="/assets/images/kobieta_i_swieca_2_3_luiza.webp" loading="lazy" alt="Woman and candle"/>
        </div>
        <div class="bg-(--bg-sec)">
        <div class="space-y-6 py-10 px-[10%] bg-[url(/assets/images/liscie_o2.webp)] bg-center bg-size-[200%] bg-no-repeat">
            <p class="font-gistesy text-[7vw] md:text-4xl">"There’s a fire in you..."</p>
            <p class="font-gistesy text-[7vw] md:text-4xl text-center">"Let’s turn it into pure wild magnetism."</p>
            <div class="flex justify-center">
                <button class="text-nowrap text-xl bg-(--bg-rose) h-auto rounded-full p-4 shadow-2xl ">"I'm So Ready to Rise!"</button>
            </div>
        </div>
        </div>
    }
}
