use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize)]
pub struct Ticket {
	pub id: u64,
	pub cid: u64, // creator user_id
	pub title: String,
}

#[derive(Deserialize)]
pub struct TicketForCreate {
	pub title: String,
}
