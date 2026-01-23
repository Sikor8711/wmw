use crate::components::footer::Footer;
use crate::components::navbar::NavBar;
use leptos::prelude::*;
use leptos_router::components::Outlet;

#[component]
pub fn MainLayout() -> impl IntoView {
    view! {
        <div class="Layout-wrapper">
            <header>
                <NavBar />
            </header>
            <main>
                <Outlet />
            </main>
            <footer>
                <Footer />
            </footer>
        </div>
    }
}
