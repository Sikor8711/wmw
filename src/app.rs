use crate::components::layout::MainLayout;
use crate::pages::about::AboutPage;
use crate::pages::home::HomePage;
use crate::pages::optin::OptIn;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes},
    path, // Helper for type-safe paths if needed, though strings work mostly
};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Link rel="preload" href="/assets/images/luiza_bnner.webp" as_="image" type_="image/webp" />
        <Link rel="preload" href="/assets/fonts/gistesy/Gistesy.woff2" as_="font" type_="font/woff2" attr:crossorigin="anonymous" />
        <Link rel="preload" href="/assets/fonts/libre_baskerville/LibreBaskerville-Regular.woff2" as_="font" type_="font/woff2" attr:crossorigin="anonymous" />
        <Link rel="preload" href="/assets/fonts/libre_baskerville/LibreBaskerville-Bold.woff2" as_="font" type_="font/woff2" attr:crossorigin="anonymous" />
        <Link rel="preload" href="/assets/fonts/libre_baskerville/LibreBaskerville-Italic.woff2" as_="font" type_="font/woff2" attr:crossorigin="anonymous" />

        // injects a stylesheet into the document <head>
        <Stylesheet id="leptos" href="/pkg/wmw.css" />

        // Paddle Script
        <Script src="https://cdn.paddle.com/paddle/v2/paddle.js"/>

        // sets the document title
        <Title text="Wildly Magnetic" />

        // Plausible Analytics
        <Script async_="true" src="https://stats.wildlymagnetic.co/js/pa-eKCu2gGzHLm1C_y1Bc66Q.js"></Script>
        <Script>
        "
        window.plausible=window.plausible||function(){(plausible.q=plausible.q||[]).push(arguments)},plausible.init=plausible.init||function(i){plausible.o=i||{}};
        plausible.init()
        "
        </Script>

        // Router
        <Router>
            <main>
                <Routes fallback=move || "Not found.">
                    // NOTE: Leptos 0.7 Router syntax simplification
                    // No need for StaticSegment wrapper anymore
                    <ParentRoute path=path!("") view=MainLayout>
                        <Route path=path!("") view=HomePage />
                        <Route path=path!("about") view=AboutPage />
                        <Route path=path!("optin") view=OptIn />
                        <Route path=path!("*any") view=NotFound />
                    </ParentRoute>
                </Routes>
            </main>
        </Router>

        // Mautic Tracking
        <Script>
            "
                (function(w,d,t,u,n,a,m){w['MauticTrackingObject']=n;
                w[n]=w[n]||function(){(w[n].q=w[n].q||[]).push(arguments)},a=d.createElement(t),
                m=d.getElementsByTagName(t)[0];a.async=1;a.src=u;m.parentNode.insertBefore(a,m)
                })(window,document,'script','https://m.wildlymagnetic.co/mtc.js','mt');
                mt('send', 'pageview');
            "
        </Script>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    #[cfg(feature = "ssr")]
    {
        // 1. Get the Axum Response Options
        if let Some(resp) = use_context::<leptos_axum::ResponseOptions>() {
            // 2. Set the status code using generic http or axum::http
            resp.set_status(axum::http::StatusCode::NOT_FOUND);
        }
    }

    view! { <h1>"Not Found"</h1> }
}
