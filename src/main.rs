#![recursion_limit = "512"]

#[cfg(feature = "ssr")]
use wmw::state::AppState;

// ❌ DELETED: mod webhook;  <-- These were the cause of the errors!
// ❌ DELETED: mod api;      <-- They belong to lib.rs, not main.rs

// --- SSR IMPORTS ---
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

// --- SSR MAIN FUNCTION ---
#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    // Load .env
    dotenvy::dotenv().ok();

    // 1. Setup Logging & Crypto
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .ok();

    // 2. Load Configuration
    let conf = leptos::config::get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let site_root = leptos_options.site_root.clone();

    // 3. Load CSS
    let css_path = format!("{site_root}/pkg/wmw.css");
    let css_content = std::fs::read_to_string(&css_path).unwrap_or_else(|_| {
        eprintln!("Failed to load CSS from {css_path}");
        "".to_string()
    });

    // 4. Setup State
    let paddle_api_key = env::var("PADDLE_API_KEY").expect("PADDLE_API_KEY must be set in .env");

    let state = AppState {
        leptos_options: leptos_options.clone(),
        paddle_key: paddle_api_key,
    };

    // 5. Define the Shell
    let shell = {
        let leptos_options = leptos_options.clone();
        let css = css_content.clone();
        move || {
            view! {
                <!DOCTYPE html>
                <html lang="en">
                    <head>
                        <meta charset="utf-8"/>
                        <meta name="viewport" content="width=device-width, initial-scale=1"/>
                        <AutoReload options=leptos_options.clone() />
                        <HydrationScripts options=leptos_options.clone()/>
                        <MetaTags/>
                        <style>{css.clone()}</style>
                    </head>
                    <body>
                        <App/>
                    </body>
                </html>
            }
        }
    };

    // 6. Define Routes
    let routes = generate_route_list(App);

    // 7. Build the Axum Router
    let app = Router::new()
        .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
        // API Routes - ✅ Using wmw::... because they are in the library
        .route("/api/create-payment", post(wmw::api::ssr::create_payment))
        .route("/webhooks/paddle", post(wmw::webhook::ssr::paddle_callback))
        .route("/favicon.ico", get(favicon))
        // Static Files
        .nest_service("/pkg", ServeDir::new(format!("{site_root}/pkg")))
        .nest_service("/assets", ServeDir::new(&*site_root))
        // Leptos Logic
        .leptos_routes(&state, routes, shell.clone())
        .fallback(move |req: Request| {
            let shell = shell.clone();
            async move {
                let handler = leptos_axum::render_app_to_stream(shell);
                handler(req).await.into_response()
            }
        })
        .with_state(state);

    // 8. Run
    println!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    // Fix Type Inference: Explicitly capture the connection info type
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}

// --- HELPER HANDLERS ---

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
