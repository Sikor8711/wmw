use crate::components::layout::MainLayout;
use crate::pages::about::AboutPage;
use crate::pages::home::HomePage;
use crate::pages::optin::OptIn;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes},
    StaticSegment, WildcardSegment,
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
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/wmw.css" />

        // sets the document title
        <Title text="Wildly Magnetic" />
        <script async src="https://stats.wildlymagnetic.co/js/pa-eKCu2gGzHLm1C_y1Bc66Q.js"></script>
        <Script>
        "
        window.plausible=window.plausible||function(){(plausible.q=plausible.q||[]).push(arguments)},plausible.init=plausible.init||function(i){plausible.o=i||{}};
        plausible.init()
        "
        </Script>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=move || "Not found.">
                    <ParentRoute path=StaticSegment("") view=MainLayout>
                        <Route path=StaticSegment("") view=HomePage />
                        <Route path=StaticSegment("about") view=AboutPage />
                        <Route path=StaticSegment("optin") view=OptIn />
                        <Route path=WildcardSegment("any") view=NotFound />
                    </ParentRoute>
                </Routes>
            </main>
        </Router>
        <script>
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

/// 404 - Not Founattr:d
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! { <h1>"Not Found"</h1> }
}
