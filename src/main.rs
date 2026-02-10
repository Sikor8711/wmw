#![recursion_limit = "512"]

use wmw::state::AppState;
#[cfg(feature = "ssr")]
mod webhook;

#[cfg(feature = "ssr")]
mod api;

// --- SSR MAIN FUNCTION ---
#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::{
        body::Body,
        extract::State,
        response::{IntoResponse, Response},
        routing::{get, post},
        Router,
    };
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use leptos_meta::MetaTags;
    use std::env;
    use tower_http::services::ServeDir;
    use wmw::app::*;
    dotenvy::dotenv().ok();

    // 1. Setup Logging & Env
    // Note: rustls crypto provider setup is good to keep
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .ok();

    // Optional: simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");

    // 2. Load Configuration
    let conf = leptos::config::get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let site_root = leptos_options.site_root.clone();

    // 3. Load CSS (Your custom logic)
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

    // 5. Define the Shell (HTML Wrapper)
    // We create a closure that captures the CSS content
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
                        <style>
                            {css.clone()}
                        </style>
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
        .nest_service("/pkg", ServeDir::new(format!("{site_root}/pkg")))
        .nest_service("/assets", ServeDir::new(format!("{site_root}/assets")))
        .route("/favicon.ico", get(favicon))
        // FIX 1: Pass &leptos_options here, NOT &state
        .leptos_routes(&leptos_options, routes, shell.clone())
        // FIX 2: Use axum::extract::Request to avoid generic type confusion
        .fallback(move |req: axum::extract::Request| {
            let options = leptos_options.clone();
            let shell = shell.clone();
            async move {
                let handler = leptos_axum::render_app_to_stream(shell);
                handler(req).await.into_response()
            }
        })
        .with_state(AppState);

    // 8. Run the Server
    println!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

// --- HELPER HANDLERS ---

#[cfg(feature = "ssr")]
async fn favicon(
    axum::extract::State(options): axum::extract::State<leptos::config::LeptosOptions>,
) -> impl axum::response::IntoResponse {
    let site_root = &options.site_root;
    let path = format!("{site_root}/favicon.ico");

    // ServeFile handles opening and streaming the file
    match tower_http::services::fs::ServeFile::new(path)
        .try_call(axum::http::Request::new(axum::body::Body::empty()))
        .await
    {
        Ok(res) => res.into_response(),
        Err(_) => (axum::http::StatusCode::NOT_FOUND, "Favicon not found").into_response(),
    }
}

// --- CLIENT SIDE ENTRY POINTS (Unchanged) ---

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
