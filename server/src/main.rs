use self::{
    error::{Error, Result},
    log::log_request,
    model::ModelController,
    web::{mw_auth, routes_login, routes_tickets},
};
use axum::{
    extract::{Path, Query},
    http::{Method, Uri},
    middleware,
    response::{Html, IntoResponse, Response},
    routing::{get, get_service},
    Json, Router,
};
use colored::Colorize;
use ctx::Ctx;
use serde::Deserialize;
use serde_json::json;
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use tracing::{debug, info};
use tracing_subscriber::EnvFilter;
use uuid::Uuid;

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
        .merge(routes_home())
        .merge(routes_hello())
        .merge(routes_login::routes())
        .nest("/api", routes_api)
        .layer(middleware::map_response(main_response_mapper))
        .layer(middleware::from_fn_with_state(
            mc.clone(),
            mw_auth::mw_ctx_resolver,
        ))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    info!("Listening on http://{addr}\n");
    axum::serve(listener, routes).await.unwrap();

    Ok(())
}

async fn main_response_mapper(
    ctx: Option<Ctx>,
    uri: Uri,
    req_method: Method,
    res: Response,
) -> Response {
    debug!(
        "{:<12} - main_response_mapper",
        "RES_MAPPER".bold().yellow()
    );

    let uuid = Uuid::new_v4();

    let service_error = res.extensions().get::<Error>();
    let client_status_error = service_error.map(|se| se.client_status_and_error());

    let error_response = client_status_error
        .as_ref()
        .map(|(status_code, client_error)| {
            let client_error_body = json!({
                "error": {
                    "type": client_error.as_ref(),
                    "req_uuid": uuid.to_string(),
                }
            });

            debug!("client_error_body: {}", client_error_body);

            (*status_code, Json(client_error_body)).into_response()
        });

    // Build and log the server log line.
    let client_error = client_status_error.unzip().1;

    // TODO: Need to hander if log_request fail (but should not fail request)
    let _ = log_request(uuid, req_method, uri, ctx, service_error, client_error).await;

    debug!("============\n");
    error_response.unwrap_or(res)
}

fn routes_home() -> Router {
    Router::new().route("/", get(handler_home))
}

fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2))
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

async fn handler_home() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

#[derive(Debug, Deserialize)]
struct Hello {
    name: Option<String>,
}

// e.g. http://localhost:3000/hello?name=foo
async fn handler_hello(Query(param): Query<Hello>) -> impl IntoResponse {
    let name = param.name.as_deref().unwrap_or("handler_hello");
    debug!("{:<12} - handler_hello - {}", "HANDLER".bold().blue(), name);
    Html(format!("<strong>{}</strong>", name))
}

// e.g. http://localhost:3000/hello2/foo
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    debug!(
        "{:<12} - handler_hello2 - {}",
        "HANDLER".bold().blue(),
        name
    );
    Html(format!("<strong>{}</strong>", name))
}
