use crate::components::utils::NewsForm;
use crate::components::utils::TagButton;
use leptos::prelude::*;
#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        // <div class="max-w-225 mx-auto">
        //     <div class="relative m-7">
        //         <div class="absolute right-0 bottom-0 size-[85%] bg-[#eae2dc] -z-100"></div>
        //         <div class="-z-10 mask-t-from-0% mask-b-to-100% bg-[url(/assets/images/luiza_bnner.png)] bg-contain bg-no-repeat">
        //             <img
        //                 src="/assets/images/luiza_bnner.png"
        //                 alt="Luiza"
        //                 class="-z-10 max-w-[80%]"
        //             />
        //         </div>
        //         <img
        //             src="/assets/images/luiza_bnner.png"
        //             alt="Luiza"
        //             class="absolute top-0 left-0 max-w-[80%] opacity-80 brightness-110"
        //         />
        //         <div class="-translate-x-4 -translate-y-2 z-100 font-gistesy -rotate-8 atext md:text-8xl text-nowrap">
        //             <h1 class="opacity-50 z-100 text-center">
        //                 "Wild hearts don’t follow rules."
        //             </h1>
        //             <h2 class="text-center z-100">"they rewrite them."</h2>
        //         </div>
        //     </div>
        // </div>
        <div class="max-w-225 mx-auto">
            <div class=" m-7">
                <img
                    src="/assets/images/luiza_bnner3.avif"
                    alt="Luiza"
                    class="max-w-[80%] shadow-[20vw_20vw_0] shadow-[#eae2dc]"
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
                <div class="grid grid-cols-2 content-center mx-2">
                    <div class="place-self-center h-25 w-30 mr-5" src="/#" alt="">
                        <img class="h-25 w-30" src="/#" alt=""/>
                    </div>
                    <div class="place-self-center">
                        <div class="h-full flex items-center">
                            <img class="max-w-8 mr-0.5" src="../../assets/images/button_deco.png" alt=""/>
                            <button class="text-nowrap text-[0.8rem] bg-white h-auto rounded-3xl p-2 shadow-2xl ">"Let's go WILD!"</button>
                            <img class="scale-x-[-1] max-w-8 ml-0.5" src="../../assets/images/button_deco.png" alt=""/>
                        </div>
                    </div>
                </div>
            </div>
        </div>
        <div>
            <p class="py-8 text-center font-gistesy md:text-5xl text-[8vw] text-nowrap">"You’re freaking magnetic and you know it."</p>
        </div>
        <TagButton bname="FREE GUIDE" />
        <div class="bg-[url(/assets/images/pokoj.png)] bg-center bg-no-repeat bg-cover ">
            <div class="grid grid-cols-2 pt-3 bg-white/80">
                <div class="place-self-center pl-2 w-[90%]">
                    <h2 class="text-[1rem] pb-3 text-center">"The Magnetic Message"</h2>
                    <p class="text-[0.8rem] text-center">"A soulful guid to finding the message that your dream clients can feel - and can't resist"</p>
                    <div class="flex">
                        <NewsForm/>
                    </div>
                </div>
                <div class="pt-3 pr-2">
                    <img src="/assets/images/monitorki.png" alt=""/>
                </div>
            </div>
        </div>
        <div class="pt-8">
            <div class="h-14 bg-[#eae2dc]"> </div>
        </div>
        <div class="space-y-10 bg-[url(/assets/images/liscie_o2.avif)] bg-center ">
            <h3 class="text-[5vw] md:text-3xl text-center pt-10">"Meet Luiza - the soul behind the"
            <br/>
            <span class="font-gistesy text-[7vw] md:text-4xl text-nowrap">"Wildly Magnetic"</span></h3>
            <img class="w-[65%] mx-auto" src="../../assets/images/luiza_w_marynarce3.avif" alt=""/>
            <p class="px-[10%] text-justify">
                "A woman who didn’t find herself in perfection, but in the mess of real life — in exhaustion, in starting over, in choosing herself again and again. She carries the depth, yes, but also the earth — the late nights, the quiet rebuild, the courage to walk away from a life that drained her."
            </p>
            <p class="px-[10%] text-justify">
                "Her work is where soul meets truth, guiding you back to the version of yourself your body remembers, your heart misses, and your future is already calling in."
            </p>
            <div class="mx-[10%] bg-[#eae2dc] text-[3vw]/[5vw] md:text-2xl text-center space-y-6 py-6">
                <p class="">
                    "You weren’t born to follow rules."
                    <br/>
                    "You were born to lead with soul."
                </p>
                <p class="font-gistesy text-[5vw] md:text-4xl">"Let’s get you there."</p>
            </div>
            <div class="flex justify-center">
                <button class="text-nowrap text-xl bg-[#D4B3AB] h-auto rounded-full p-4 shadow-2xl ">"Activate My Wild Potential!"</button>
            </div>
        </div>
    }
}
