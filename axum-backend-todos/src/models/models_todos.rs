use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone)]
pub struct Todo {
    pub id: i32,
    pub text: String,
    pub completed: bool,
}

// The query parameters for todos index
#[derive(Debug, Deserialize, Default)]
pub struct Pagination {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTodo {
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTodo {
    pub text: Option<String>,
    pub completed: Option<bool>,
}
