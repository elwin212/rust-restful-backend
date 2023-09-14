use crate::{Error, Result, web};

use axum::{ Json, Router, routing::post};
use serde::Deserialize;
use serde_json:: {json, Value };
use tower_cookies::{Cookies, Cookie};


pub fn routes() -> Router {
    Router::new()
        .route("/api/login", post(api_login))
}

async fn api_login(cookies: Cookies,payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:<12} - api_login - {payload:?}", "HANDLER");

    if payload.username != "demo1" || payload.password != "testpwd" {
        return Err(Error::LoginFail);
    }

    cookies.add(Cookie::new(web::AUTH_TOKEN, "user1.exp.sign"));
    // TODO: Set cookies

    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    Ok(body)

}

#[derive(Debug, Deserialize)]
pub struct LoginPayload {
    username: String,
    password: String,
}