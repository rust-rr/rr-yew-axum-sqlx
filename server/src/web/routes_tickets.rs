use crate::{
    ctx::Ctx,
    error::Result,
    model::{ModelController, Ticket, TicketForCreate},
};
use axum::{
    extract::{Path, State},
    routing::{delete, post},
    Json, Router,
};
use colored::Colorize;
use tracing::debug;

pub fn routes(mc: ModelController) -> Router {
    Router::new()
        .route("/tickets", post(create_ticket).get(list_tickets))
        .route("/tickets/:id", delete(delete_ticket))
        .with_state(mc)
}

async fn create_ticket(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Json(ticket_fc): Json<TicketForCreate>,
) -> Result<Json<Ticket>> {
    debug!("{:<12} - create_ticket", "HANDLER".bold().blue());

    let ticket = mc.create_ticket(ctx, ticket_fc).await?;

    Ok(Json(ticket))
}

async fn list_tickets(
    State(mc): State<ModelController>,
    ctx: Ctx,
) -> Result<Json<Vec<Ticket>>> {
    debug!("{:<12} - list_tickets", "HANDLER".bold().blue());

    let tickets = mc.list_tickets(ctx).await?;

    Ok(Json(tickets))
}

async fn delete_ticket(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Path(id): Path<u64>,
) -> Result<Json<Ticket>> {
    debug!("{:<12} - delete_ticket", "HANDLER".bold().blue());

    let ticket = mc.delete_ticket(ctx, id).await?;

    Ok(Json(ticket))
}
