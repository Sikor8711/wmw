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
        <script type="text/javascript">
            "
            (function(w,d,t,u,n,a,m){w['MauticTrackingObject']=n;
            w[n]=w[n]||function(){(w[n].q=w[n].q||[]).push(arguments)},a=d.createElement(t),
            m=d.getElementsByTagName(t)[0];a.async=1;a.src=u;m.parentNode.insertBefore(a,m)
            })(window,document,'script','https://m.wildlymagnetic.co/mtc.js','mt');
            mt('send', 'pageview');
            "
        </script>
    }
}
