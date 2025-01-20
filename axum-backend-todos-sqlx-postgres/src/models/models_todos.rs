use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

#[derive(Debug, Serialize, FromRow)]
pub struct Todo {
    pub id: i32,
    pub text: String,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreateTodo {
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTodo {
    pub text: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ApiError {
    pub error: String,
}
