use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, patch},
    Json, Router,
};

use crate::{
    models::models_todos::{CreateTodo, Pagination, Todo, UpdateTodo},
    utils::db::DbArcRwLock,
};

// * curl -X POST http://localhost:8000/todos -H "Content-Type: application/json" -d '{"text": "Buy groceries"}'
// * curl -X PATCH http://localhost:8000/todos/1 -H "Content-Type: application/json" -d '{"text": "Buy more groceries"}'
// * curl -X DELETE http://localhost:8000/todos/1

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                        🦀 MAIN 🦀                          */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

pub fn service_todos() -> Router<DbArcRwLock> {
    Router::new()
        .route("/todos", get(todos_index).post(todos_create))
        .route("/todos/{id}", patch(todos_update).delete(todos_delete))
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

pub async fn todos_index(
    pagination: Query<Pagination>,
    State(db): State<DbArcRwLock>,
) -> impl IntoResponse {
    let todos = db.read().unwrap();

    let todos = todos
        .values()
        .skip(pagination.offset.unwrap_or(0))
        .take(pagination.limit.unwrap_or(usize::MAX))
        .cloned()
        .collect::<Vec<_>>();

    Json(todos)
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

pub async fn todos_create(
    State(db): State<DbArcRwLock>,
    Json(input): Json<CreateTodo>,
) -> impl IntoResponse {
    let mut todos = db.write().unwrap();
    let id = todos.len() as i32 + 1;

    let todo = Todo {
        id,
        text: input.text,
    };

    todos.insert(todo.id, todo.clone());

    (StatusCode::CREATED, Json(todo))
}

pub async fn todos_update(
    Path(id): Path<i32>,
    State(db): State<DbArcRwLock>,
    Json(input): Json<UpdateTodo>,
) -> Result<impl IntoResponse, StatusCode> {
    let mut todo = db
        .read()
        .unwrap()
        .get(&id)
        .cloned()
        .ok_or(StatusCode::NOT_FOUND)?;

    if let Some(text) = input.text {
        todo.text = text;
    }

    db.write().unwrap().insert(todo.id, todo.clone());

    Ok(Json(todo))
}

pub async fn todos_delete(Path(id): Path<i32>, State(db): State<DbArcRwLock>) -> impl IntoResponse {
    if db.write().unwrap().remove(&id).is_some() {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}
