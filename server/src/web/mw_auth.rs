use super::AUTH_TOKEN;
use crate::{
    ctx::Ctx,
    error::{Error, Result},
    model::ModelController,
};
use async_trait::async_trait;
use axum::{
    body::Body,
    extract::{FromRequestParts, State},
    http::{request::Parts, Request},
    middleware::Next,
    response::Response,
};
use colored::Colorize;
use lazy_regex::regex_captures;
use tower_cookies::{Cookie, Cookies};

pub async fn mw_require_auth(ctx: Result<Ctx>, req: Request<Body>, next: Next) -> Result<Response> {
    println!(
        "->> {:<12} - mw_require_auth - {ctx:?}",
        "MIDDLEWARE".bold().green()
    );

    ctx?;

    Ok(next.run(req).await)
}

pub async fn mw_ctx_resolver(
    _mc: State<ModelController>,
    cookies: Cookies,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response> {
    println!("->> {:<12} - mw_ctx_resolver", "MIDDLEWARE".bold().green());

    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());
    let result_ctx = match auth_token
        .ok_or(Error::AuthFailedNoAuthTokenCookie)
        .and_then(parse_token)
    {
        Ok((user_id, _exp, _sign)) => Ok(Ctx::new(user_id)),
        Err(err) => Err(err),
    };

    // Remove the cookie if something went wrong other than NoAuthTokenCookie.
    if result_ctx.is_err() && !matches!(result_ctx, Err(Error::AuthFailedNoAuthTokenCookie)) {
        cookies.remove(Cookie::from(AUTH_TOKEN))
    }

    // Store the ctx_result in the request extension.
    req.extensions_mut().insert(result_ctx);

    Ok(next.run(req).await)
}

#[async_trait]
impl<S> FromRequestParts<S> for Ctx
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        println!("->> {:<12} - Ctx", "EXTRACTOR".bold().purple());

        parts
            .extensions
            .get::<Result<Ctx>>()
            .ok_or(Error::AuthFailedCtxNotInRequestExt)?
            .clone()
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
