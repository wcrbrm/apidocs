pub mod metrics;
pub mod openapi;
pub mod prelude;

use axum::response::*;
use axum::{extract::DefaultBodyLimit, extract::Extension, routing::*, Router, Server};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::limit::*;
use tower_http::trace::*;
use tracing::*;

pub struct AppState {
    pub storage_path: String,
}

pub async fn handle_webroot() -> impl IntoResponse {
    "# API DOCS".into_response()
}

#[allow(unused_imports)]
use axum::ServiceExt;

pub async fn run(
    socket_addr: SocketAddr,
    storage_path: String,
) -> anyhow::Result<()> {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let shared_state = Arc::new(AppState {
        storage_path,
    });
    let app = Router::new();
    let a = app
        .layer(cors)
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
        .route("/apidocs/openapi.json", get(openapi::handle))
        .layer(Extension(shared_state)) 
        .route("/apidocs/", get(handle_webroot));

    info!("Listening on {}", socket_addr);
    Server::bind(&socket_addr)
        .serve(a.into_make_service_with_connect_info::<SocketAddr>())
        .await?;
    Ok(())
}
