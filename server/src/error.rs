use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use colored::Colorize;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub enum Error {
    LoginFailed,
    TicketDeleteFailedIdNotFound { id: u64 },
    AuthFailedNoAuthTokenCookie,
    AuthFailedTokenWrongFormat,
    AuthFailedCtxNotInRequestExt,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("->> {:<12} - {self:?}", "INTO_RES".bold().red());

        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()
    }
}
