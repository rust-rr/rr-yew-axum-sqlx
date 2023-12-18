use crate::{ctx::Ctx, error::Error, log::log_request};
use axum::{
    http::{Method, Uri},
    response::{IntoResponse, Response},
    Json,
};
use colored::Colorize;
use serde_json::json;
use tracing::debug;
use uuid::Uuid;

pub async fn main_response_mapper(
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
