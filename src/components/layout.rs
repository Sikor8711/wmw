use crate::components::footer::Footer;
use crate::components::navbar::NavBar;
use leptos::prelude::*;
use leptos_router::components::Outlet;

#[component]
pub fn MainLayout() -> impl IntoView {
    view! {
    <div class="Layout-wrapper">
        <header class="sticky top-0 z-200">
            <NavBar />
        </header>
        <main class="pt-26">
            <Outlet />
        </main>
        <footer>
            <Footer />
        </footer>
    </div>
    }
}
