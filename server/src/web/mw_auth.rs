use crate::error::{Error, Result};
use axum::{body::Body, http::Request, middleware::Next, response::Response};
use colored::Colorize;
use tower_cookies::Cookies;

use super::AUTH_TOKEN;

pub async fn mw_require_auth(cookies: Cookies, req: Request<Body>, next: Next) -> Result<Response> {
    println!("->> {:<12} - mw_require_auth", "MIDDLEWARE".bold().green());

    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    // TODO: Real auth-token parsing & validation.
    auth_token.ok_or(Error::AuthFailedNoAuthTokenCookie)?;

    Ok(next.run(req).await)
}
