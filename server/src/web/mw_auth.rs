use super::AUTH_TOKEN;
use crate::{
    ctx::Ctx,
    error::{Error, Result},
};
use async_trait::async_trait;
use axum::{
    body::Body,
    extract::FromRequestParts,
    http::{request::Parts, Request},
    middleware::Next,
    response::Response,
    RequestPartsExt,
};
use colored::Colorize;
use lazy_regex::regex_captures;
use tower_cookies::Cookies;

pub async fn mw_require_auth(ctx: Result<Ctx>, req: Request<Body>, next: Next) -> Result<Response> {
    println!("->> {:<12} - mw_require_auth", "MIDDLEWARE".bold().green());

    ctx?;

    Ok(next.run(req).await)
}
#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        println!("->> {:<12} - Ctx", "EXTRACTOR".bold().purple());

        let cookies: Cookies = parts.extract::<Cookies>().await.unwrap();
        let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());
        let (user_id, exp, sign) = auth_token
            .ok_or(Error::AuthFailedNoAuthTokenCookie)
            .and_then(parse_token)?;

        Ok(Ctx::new(user_id))
    }
}

/// Parse a token of format `user-[user-id].[expiration]-[signature]`
/// Return (user-id, expiration, signature)
fn parse_token(token: String) -> Result<(u64, String, String)> {
    let (_whole, user_id, exp, sign) = regex_captures!(r#"^user-(\d+)\.(.+)\.(.+)"#, &token)
        .ok_or(Error::AuthFailedTokenWrongFormat)?;

    let user_id = user_id
        .parse()
        .map_err(|_| Error::AuthFailedTokenWrongFormat)?;

    Ok((user_id, exp.to_string(), sign.to_string()))
}
