#![allow(unused)]

pub use self::error::{Error, Result};

use std::net::SocketAddr;
use axum::{Router, middleware};
use axum::extract::{Query, Path};
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, get_service};
use serde::Deserialize;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

mod error;
mod web;

#[tokio::main]
async fn main() {
    let routes_all = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .layer(middleware::map_response(main_resp_mapper))
        .layer(CookieManagerLayer::new())
        .fallback_service(route_static());    

    // region:      --- Start Server ---
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!(
        "Listening on http://{}",
        addr
    );
    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();
    // endregion:   --- Start Server ---
}

async fn main_resp_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_resp_mapper - {res:?}", "RES_MAPPER");

    println!();
    res

}

fn route_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

// region:      --- Routes Hello ---
fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World!");

    Html(format!("<h1>Hello, <strong>{name}</strong></h1>"))
}

async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello2 - {name:?}", "HANDLER");

    Html(format!("<h1>Hello, <strong>{name}</strong></h1>"))
}


// endregion:   --- Routes Hello ---