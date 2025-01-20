use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow)]
pub struct Todo {
    pub id: i32,
    pub text: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
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
