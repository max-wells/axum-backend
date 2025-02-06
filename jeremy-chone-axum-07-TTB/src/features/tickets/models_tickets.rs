use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

use crate::ctx::Ctx;
use crate::{Error, Result};

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
