use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, patch},
    Json, Router,
};

use crate::{
    models::models_todos::{ApiError, CreateTodo, Todo, UpdateTodo},
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
    let todos = sqlx::query_as!(Todo, "SELECT * FROM todos ORDER BY created_at DESC")
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
    let todo = sqlx::query_as!(
        Todo,
        "INSERT INTO todos (text) VALUES ($1) RETURNING id, text, created_at, updated_at",
        input.text
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(todo)))
}

pub async fn todos_update(
    Path(id): Path<i32>,
    State(pool): State<Db>,
    Json(input): Json<UpdateTodo>,
) -> Result<impl IntoResponse, (StatusCode, Json<ApiError>)> {
    if let Some(text) = input.text {
        let todo = sqlx::query_as!(
            Todo,
            "UPDATE todos SET text = $1, updated_at = NOW() WHERE id = $2 RETURNING *",
            text,
            id
        )
        .fetch_one(&pool)
        .await
        .map_err(|err| {
            if err.to_string().contains("no rows") {
                (
                    StatusCode::NOT_FOUND,
                    Json(ApiError {
                        error: format!("Todo with id {} not found", id),
                    }),
                )
            } else {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiError {
                        error: "Internal server error".to_string(),
                    }),
                )
            }
        })?;

        Ok(Json(todo))
    } else {
        Err((
            StatusCode::BAD_REQUEST,
            Json(ApiError {
                error: "Text field is required".to_string(),
            }),
        ))
    }
}

pub async fn todos_delete(
    Path(id): Path<i32>,
    State(pool): State<Db>,
) -> Result<impl IntoResponse, (StatusCode, Json<ApiError>)> {
    let result = sqlx::query!("DELETE FROM todos WHERE id = $1", id)
        .execute(&pool)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiError {
                    error: "Internal server error".to_string(),
                }),
            )
        })?;

    if result.rows_affected() == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            Json(ApiError {
                error: format!("Todo with id {} not found", id),
            }),
        ));
    }

    Ok(StatusCode::NO_CONTENT)
}
