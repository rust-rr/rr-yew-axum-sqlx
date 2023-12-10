use axum::{
    extract::{Path, Query},
    middleware,
    response::{Html, IntoResponse, Response},
    routing::{get, get_service},
    Router,
};
use serde::Deserialize;
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

use web::routes_login;

mod error;
mod web;

#[tokio::main]
async fn main() {
    let routes = Router::new()
        .merge(routes_home())
        .merge(routes_hello())
        .merge(routes_login::routes())
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    // cargo watch -q -c -w src/ -x run
    println!("Listening on http://{addr}\n");
    axum::serve(listener, routes).await.unwrap();
}

async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");
    res
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
    println!("->> {:<12} - handler_hello - {}", "HANDLER", name);
    Html(format!("<strong>{}</strong>", name))
}

// e.g. http://localhost:3000/hello2/foo
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello2 - {}", "HANDLER", name);
    Html(format!("<strong>{}</strong>", name))
}
