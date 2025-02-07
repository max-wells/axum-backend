use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize)]
pub struct Ticket {
	pub id: u64,
	pub creator_user_id: u64,
	pub title: String,
}

#[derive(Deserialize)]
pub struct TicketForCreate {
	pub title: String,
}
