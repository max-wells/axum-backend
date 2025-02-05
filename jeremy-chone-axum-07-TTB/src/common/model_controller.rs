use std::sync::{Arc, Mutex};

use crate::common::ctx::Ctx;
use crate::features::tickets::models_tickets::{Ticket, TicketForCreate};
use crate::{MyError, MyResult};

#[derive(Clone)]
pub struct ModelController {
	tickets_store: Arc<Mutex<Vec<Option<Ticket>>>>,
}

impl ModelController {
	pub async fn new() -> MyResult<Self> {
		Ok(Self {
			tickets_store: Arc::default(),
		})
	}
}

impl ModelController {
	pub async fn create_ticket(
		&self,
		ctx: Ctx,
		ticket_fc: TicketForCreate,
	) -> MyResult<Ticket> {
		let mut store = self.tickets_store.lock().unwrap();

		let id = store.len() as u64;
		let ticket = Ticket {
			id,
			cid: ctx.user_id(),
			title: ticket_fc.title,
		};
		store.push(Some(ticket.clone()));

		Ok(ticket)
	}

	pub async fn list_tickets(&self, _ctx: Ctx) -> MyResult<Vec<Ticket>> {
		let store = self.tickets_store.lock().unwrap();

		let tickets = store.iter().filter_map(|t| t.clone()).collect();

		Ok(tickets)
	}

	pub async fn delete_ticket(&self, _ctx: Ctx, id: u64) -> MyResult<Ticket> {
		let mut store = self.tickets_store.lock().unwrap();

		let ticket = store.get_mut(id as usize).and_then(|t| t.take());

		ticket.ok_or(MyError::TicketDeleteFailIdNotFound { id })
	}
}
