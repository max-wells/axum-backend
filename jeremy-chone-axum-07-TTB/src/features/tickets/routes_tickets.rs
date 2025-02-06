use axum::extract::{Path, State};
use axum::routing::{delete, post};
use axum::{Json, Router};

use crate::common::ctx::Ctx;
use crate::MyResult;

use crate::common::model_controller::ModelController;
use crate::features::tickets::models_tickets::{Ticket, TicketForCreate};

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                        🦀 MAIN 🦀                          */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

pub fn routes_tickets(model_controller: ModelController) -> Router {
	Router::new()
		.route("/tickets", post(create_ticket).get(list_tickets))
		.route("/tickets/:id", delete(delete_ticket))
		.with_state(model_controller)
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

// 1. Create a ticket
async fn create_ticket(
	State(model_controller): State<ModelController>,
	ctx: Ctx,
	Json(ticket_fc): Json<TicketForCreate>,
) -> MyResult<Json<Ticket>> {
	println!("->> {:<12} - create_ticket", "HANDLER");

	let ticket = model_controller.create_ticket(ctx, ticket_fc).await?;

	Ok(Json(ticket))
}

// 2. List all tickets
async fn list_tickets(
	State(model_controller): State<ModelController>,
	ctx: Ctx,
) -> MyResult<Json<Vec<Ticket>>> {
	println!("->> {:<12} - list_tickets", "HANDLER");

	let tickets = model_controller.list_tickets(ctx).await?;

	Ok(Json(tickets))
}

// 3. Delete a ticket
async fn delete_ticket(
	State(model_controller): State<ModelController>,
	ctx: Ctx,
	Path(id): Path<u64>,
) -> MyResult<Json<Ticket>> {
	println!(">>> {:<12} - delete_ticket", "HANDLER");

	let ticket = model_controller.delete_ticket(ctx, id).await?;

	Ok(Json(ticket))
}
