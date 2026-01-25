use crate::components::home_sections::{
    FireInYou, HeroSection, ManSection, MeetLuiza, NewsForm, Stopper2, StopperSection, TagButton,
    UnlockSection, WhatIDo,
};

use leptos::prelude::*;
#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <HeroSection/>
        <ManSection/>
        <UnlockSection/>
        <TagButton bname="1h MASTERCLASS" />
        <StopperSection />
        <NewsForm/>
        <Stopper2/>
        <MeetLuiza />
        <WhatIDo />
        <FireInYou />
    }
}
