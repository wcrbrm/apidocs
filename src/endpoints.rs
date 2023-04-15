pub mod metrics;
pub mod openapi;
pub mod prelude;

use crate::svc::Config;
use axum::response::*;
use axum::{extract::DefaultBodyLimit, extract::Extension, routing::*, Router, Server};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::limit::*;
use tower_http::services::ServeDir;
use tower_http::trace::*;
use tracing::*;

pub struct AppState {
    pub storage_path: String,
    pub assets_path: String,
    pub secret_token: String,
    pub header: String,
    pub footer: String,
}

/// serve as HTML page listing all current services with
/// links to their documentation sections
pub async fn handle_webroot(Extension(state): Extension<Arc<AppState>>) -> impl IntoResponse {
    let config = Config::from_path(&state.storage_path);
    let mut html = String::new();

    html.push_str(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>API DOCS</title>
  <meta name="theme-color" content="\#000000" />
  <meta name="description" content="API documentation" />
  <style>
    a {
        display: block;
        color: blue !important;
        padding: 5px 0px;
        margin: 0px;
        text-decoration: none;
        font-size: 0.9em;
        font-weight: normal;
    }
    a:hover {
        text-decoration: underline;
    }
    .doc a:before {
        content: " 📄 ";
    }
    h1 {
        text-align: center;
        color: #404040;
    }
    h2, h3, h4 {
        color: #606060;
        margin-bottom: 0px;
    }
    * {
        font-family: 'Ubuntu', 'Segoe-UI', sans-serif;
    }
    p {
        color: #899;
        margin-top: 0px;
        margin-bottom: 2px;
        font-size: 0.75em;
    }
    .entry {
        width: 320px;
        margin: 0px auto;
    }
    footer {
        text-align: center;
        font-size: 0.8rem;
        color: #999;
        position: fixed;
        bottom: 0px;
        width: 100%;
        padding-bottom: 10px;
    }
  </style>
</head>
<body>
<h1>"#,
    );
    html.push_str(state.header.as_str());
    html.push_str("</h1>");

    html.push_str(config.into_html().as_str());
    html.push_str("<footer>");
    html.push_str(state.footer.as_str());
    html.push_str(r#"</footer></body></html>"#);
    Html(html)
}

#[allow(unused_imports)]
use axum::ServiceExt;

pub async fn run(
    socket_addr: SocketAddr,
    storage_path: String,
    assets_path: String,
    header: String,
    footer: String,
    secret_token: String,
) -> anyhow::Result<()> {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let serve_dir = ServeDir::new(&assets_path);
    // .not_found_service(ServeFile::new("./assets/index.html"));
    let shared_state = Arc::new(AppState {
        storage_path,
        assets_path,
        header,
        footer,
        secret_token,
    });
    let app = Router::new();
    let a = app
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new(16 * 1024 * 1024)) // 10mb
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(
                    DefaultMakeSpan::new()
                        .level(Level::DEBUG)
                        .include_headers(true),
                )
                .on_request(DefaultOnRequest::new().level(Level::TRACE))
                .on_response(
                    DefaultOnResponse::new()
                        .level(Level::INFO)
                        .include_headers(true),
                ),
        )
        .route("/openapi.json", get(openapi::handle))
        .route("/", get(handle_webroot))
        .layer(Extension(shared_state))
        .fallback_service(serve_dir)
        .layer(cors);

    info!("Listening on {}", socket_addr);
    Server::bind(&socket_addr)
        .serve(a.into_make_service_with_connect_info::<SocketAddr>())
        .await?;
    Ok(())
}
