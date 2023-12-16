use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use colored::Colorize;
use serde::Serialize;
use tracing::debug;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    LoginFailed,
    TicketDeleteFailedIdNotFound { id: u64 },
    AuthFailedNoAuthTokenCookie,
    AuthFailedTokenWrongFormat,
    AuthFailedCtxNotInRequestExt,
    ConfigMissingEnv(&'static str),
}

#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    LOGIN_FAIL,
    NO_AUTH,
    INVALID_PARAMS,
    SERVICE_ERROR,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        debug!("{:<12} - {self:?}", "INTO_RES".bold().red());

        // Create a placeholder Axum response.
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        // Insert the Error into the response.
        response.extensions_mut().insert(self);

        response
    }
}

impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        #[allow(unreachable_patterns)]
        match self {
            Self::LoginFailed => {
                (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL)
            }

            Self::AuthFailedCtxNotInRequestExt
            | Self::AuthFailedNoAuthTokenCookie
            | Self::AuthFailedTokenWrongFormat => {
                (StatusCode::FORBIDDEN, ClientError::NO_AUTH)
            }

            Self::TicketDeleteFailedIdNotFound { .. } => {
                (StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS)
            }

            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::SERVICE_ERROR,
            ),
        }
    }
}
