use self::{
    error::Result,
    model::ModelController,
    web::{mw_auth, mw_res_map, routes_login, routes_static, routes_tickets},
};
use axum::{middleware, Router};
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;
use tracing::info;
use tracing_subscriber::EnvFilter;

mod config;
mod ctx;
mod error;
mod log;
mod model;
mod web;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .without_time()
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // Initialize ModelController
    let mc = ModelController::new().await?;
    let routes_api = routes_tickets::routes(mc.clone())
        .route_layer(middleware::from_fn(mw_auth::mw_require_auth));

    let routes = Router::new()
        .merge(routes_login::routes())
        .nest("/api", routes_api)
        .layer(middleware::map_response(mw_res_map::main_response_mapper))
        .layer(middleware::from_fn_with_state(
            mc.clone(),
            mw_auth::mw_ctx_resolver,
        ))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static::serve_dir());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    info!("Listening on http://{addr}\n");
    axum::serve(listener, routes).await.unwrap();

    Ok(())
}
