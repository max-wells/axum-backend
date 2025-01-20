use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, patch},
    Json, Router,
};

use crate::{
    models::models_todos::{CreateTodo, Todo, UpdateTodo},
    utils::db::Db,
};

// * curl -X POST http://localhost:3000/todos -H "Content-Type: application/json" -d '{"text": "Buy groceries"}'
// * curl -X PATCH http://localhost:3000/todos/1 -H "Content-Type: application/json" -d '{"text": "Buy more groceries"}'
// * curl -X DELETE http://localhost:3000/todos/1

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                        🦀 MAIN 🦀                          */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

pub fn service_todos() -> Router<Db> {
    Router::new()
        .route("/todos", get(todos_index).post(todos_create))
        .route("/todos/{id}", patch(todos_update).delete(todos_delete))
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

pub async fn todos_index(State(pool): State<Db>) -> Result<impl IntoResponse, StatusCode> {
    let todos = sqlx::query_as::<_, Todo>("SELECT * FROM todos ORDER BY created_at DESC")
        .fetch_all(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(todos))
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

pub async fn todos_create(
    State(pool): State<Db>,
    Json(input): Json<CreateTodo>,
) -> Result<impl IntoResponse, StatusCode> {
    let todo = sqlx::query_as::<_, Todo>("INSERT INTO todos (text) VALUES ($1) RETURNING *")
        .bind(input.text)
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(todo)))
}

pub async fn todos_update(
    Path(id): Path<i32>,
    State(pool): State<Db>,
    Json(input): Json<UpdateTodo>,
) -> Result<impl IntoResponse, StatusCode> {
    if let Some(text) = input.text {
        let todo = sqlx::query_as::<_, Todo>(
            "UPDATE todos SET text = $1, updated_at = NOW() WHERE id = $2 RETURNING *",
        )
        .bind(text)
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

        Ok(Json(todo))
    } else {
        Err(StatusCode::BAD_REQUEST)
    }
}

pub async fn todos_delete(
    Path(id): Path<i32>,
    State(pool): State<Db>,
) -> Result<impl IntoResponse, StatusCode> {
    sqlx::query("DELETE FROM todos WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::NO_CONTENT)
}
