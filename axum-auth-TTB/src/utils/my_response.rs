use core::str;
use serde::{Deserialize, Serialize};

// TODO. Export it from elsewhere
#[derive(Serialize, Deserialize)]
pub struct MyResponse {
    pub status: &'static str,
    pub message: String,
}
