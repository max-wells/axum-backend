use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, patch},
    Json, Router,
};

use crate::{
    models::models_todos::{ApiError, CreateTodo, Todo, UpdateTodo},
    utils::db::AppState,
};

// * curl -X POST http://localhost:3000/todos -H "Content-Type: application/json" -d '{"text": "Buy groceries"}'
// * curl -X PATCH http://localhost:3000/todos/1 -H "Content-Type: application/json" -d '{"text": "Buy more groceries"}'
// * curl -X DELETE http://localhost:3000/todos/1

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                        🦀 MAIN 🦀                          */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

pub fn service_todos() -> Router<AppState> {
    Router::new()
        .route("/todos", get(todos_index).post(todos_create))
        .route("/todos/{id}", patch(todos_update).delete(todos_delete))
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

pub async fn todos_index(app_state: State<AppState>) -> Result<impl IntoResponse, StatusCode> {
    let todos = sqlx::query_as!(
        Todo,
        r#"
            SELECT * FROM todos 
            ORDER BY created_at DESC
        "#
    )
    .fetch_all(&app_state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(todos))
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

pub async fn todos_create(
    app_state: State<AppState>,
    Json(body): Json<CreateTodo>,
) -> Result<impl IntoResponse, StatusCode> {
    let todo = sqlx::query_as!(
        Todo,
        r#"
            INSERT INTO todos (text) 
            VALUES ($1) 
            RETURNING id, text, created_at, updated_at
        "#,
        body.text
    )
    .fetch_one(&app_state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(todo)))
}

pub async fn todos_update(
    Path(id): Path<i32>,
    app_state: State<AppState>,
    Json(body): Json<UpdateTodo>,
) -> impl IntoResponse {
    let result = sqlx::query_as!(
        Todo,
        r#"
            UPDATE todos
            SET text = COALESCE($1, text),
                updated_at = NOW()
            WHERE id = $2
            RETURNING *
        "#,
        body.text,
        id
    )
    .fetch_one(&app_state.db)
    .await;

    match result {
        Ok(todo) => Json(todo).into_response(),
        Err(sqlx::Error::RowNotFound) => (
            StatusCode::NOT_FOUND,
            Json(ApiError {
                error: format!("Todo with id {} not found", id),
            }),
        )
            .into_response(),
        Err(e) => {
            eprintln!("Failed to update todo: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiError {
                    error: format!("Failed to update todo: {}", e),
                }),
            )
                .into_response()
        }
    }
}

pub async fn todos_delete(
    Path(id): Path<i32>,
    app_state: State<AppState>,
) -> Result<impl IntoResponse, (StatusCode, Json<ApiError>)> {
    let result = sqlx::query!(
        r#"
            DELETE FROM todos 
            WHERE id = $1
        "#,
        id
    )
    .execute(&app_state.db)
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
