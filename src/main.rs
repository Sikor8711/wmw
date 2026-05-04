#![recursion_limit = "512"]

#[cfg(feature = "ssr")]
use axum::{
    body::Body,
    extract::{Request, State},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
#[cfg(feature = "ssr")]
use leptos::prelude::*;
#[cfg(feature = "ssr")]
use leptos_axum::{generate_route_list, LeptosRoutes};
#[cfg(feature = "ssr")]
use leptos_meta::MetaTags;
#[cfg(feature = "ssr")]
use std::env;
#[cfg(feature = "ssr")]
use std::net::SocketAddr;
#[cfg(feature = "ssr")]
use tower_http::services::ServeDir;
#[cfg(feature = "ssr")]
use wmw::app::*;
#[cfg(feature = "ssr")]
use wmw::state::AppState;
#[cfg(feature = "ssr")]
use time::Duration;
#[cfg(feature = "ssr")]
use tower_sessions::{Expiry, MemoryStore, Session, SessionManager};


// --- SSR MAIN FUNCTION ---
#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let conf = leptos::config::get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let site_root = leptos_options.site_root.clone();
    let css_path = format!("{site_root}/pkg/wmw.css");
    let css_content = std::fs::read_to_string(&css_path).unwrap_or_else(|_| {
        eprintln!("Failed to load CSS from {css_path}");
        "".to_string()
    });
    let paddle_api_key = env::var("PADDLE_API_KEY").expect("PADDLE_API_KEY must be set in .env");
    let state = AppState {
        leptos_options: leptos_options.clone(),
        paddle_key: paddle_api_key,
    };
    let shell = {
        let leptos_options = leptos_options.clone();
        let css = css_content.clone();
        move || {
            view! {
                <html lang="en">
                    <head>
                        <meta charset="utf-8" />
                        <meta name="viewport" content="width=device-width, initial-scale=1" />
                        <AutoReload options=leptos_options.clone() />
                        <HydrationScripts options=leptos_options.clone() />
                        <MetaTags />
                        <style>{css.clone()}</style>
                    </head>
                    <body>
                        <App />
                    </body>
                </html>
            }
        }
    };

    let routes = generate_route_list(App);
    let app = Router::new()
        .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
        .route("/api/create-payment", post(wmw::api::ssr::create_payment))
        .route("/webhooks/paddle", post(wmw::webhook::ssr::paddle_callback))
        .route("/favicon.ico", get(favicon))
        .nest_service("/pkg", ServeDir::new(format!("{site_root}/pkg")))
        .nest_service("/assets", ServeDir::new(&*site_root))
        .leptos_routes(&state, routes, shell.clone())
        .fallback(move |req: Request| {
            let shell = shell.clone();
            async move {
                let handler = leptos_axum::render_app_to_stream(shell);
                handler(req).await.into_response()
            }
        })
        .with_state(state);

    println!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}


#[cfg(feature = "ssr")]
async fn favicon(State(options): State<leptos::config::LeptosOptions>) -> impl IntoResponse {
    use axum::http::StatusCode;
    use tower::ServiceExt;
    use tower_http::services::ServeFile;

    let site_root = &options.site_root;
    let path = format!("{site_root}/favicon.ico");

    let req = Request::builder().body(Body::empty()).unwrap();

    match ServeFile::new(path).oneshot(req).await {
        Ok(res) => res.into_response(),
        Err(_) => (StatusCode::NOT_FOUND, "Favicon not found").into_response(),
    }
}

// --- CLIENT SIDE ENTRY POINTS ---

#[cfg(not(any(feature = "ssr", feature = "csr")))]
pub fn main() {
    // no client-side main function
}

#[cfg(all(not(feature = "ssr"), feature = "csr"))]
pub fn main() {
    use wmw::app::*;
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}
