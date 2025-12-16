use leptos::prelude::*;
use crate::components::navbar::NavBar;
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
        </div>
    }
}
