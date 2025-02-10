use std::sync::Arc;
use axum::{
    response::IntoResponse,
    Extension, Json,
};
use validator::Validate;

use crate::{
    common::{db::UserExt, middleware::JWTAuthMiddeware},
    domain::users::dtos::dtos::{FilterUserDto, RoleUpdateDto, UserData, UserResponseDto},
    utils::my_errors::MyHttpError,
    AppState,
};




pub async fn users_update_role(
    Extension(app_state): Extension<Arc<AppState>>,
    Extension(user): Extension<JWTAuthMiddeware>,
    Json(body): Json<RoleUpdateDto>,
) -> Result<impl IntoResponse, MyHttpError> {
    body.validate().map_err(|e| MyHttpError::bad_request(e.to_string()))?;

    let user = &user.user;

    let user_id = uuid::Uuid::parse_str(&user.id.to_string()).unwrap();

    let result = app_state
        .db_client
        .update_user_role(user_id.clone(), body.role)
        .await
        .map_err(|e| MyHttpError::server_error(e.to_string()))?;

    let filtered_user = FilterUserDto::filter_user(&result);

    let response = UserResponseDto {
        data: UserData { user: filtered_user },
        status: "success".to_string(),
    };

    Ok(Json(response))
}