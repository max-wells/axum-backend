use std::sync::Arc;
use axum::{
    response::IntoResponse,
    Extension, Json,
};
use validator::Validate;

use crate::{
    db::UserExt,
    domain::users::dtos::dtos::{FilterUserDto, NameUpdateDto, UserData,UserResponseDto},
    error::MyHttpError,
    middleware::JWTAuthMiddeware,
    AppState,
};



pub async fn users_update_name(
    Extension(app_state): Extension<Arc<AppState>>,
    Extension(user): Extension<JWTAuthMiddeware>,
    Json(body): Json<NameUpdateDto>,
) -> Result<impl IntoResponse, MyHttpError> {
    body.validate().map_err(|e| MyHttpError::bad_request(e.to_string()))?;

    let user = &user.user;

    let user_id = uuid::Uuid::parse_str(&user.id.to_string()).unwrap();

    let result = app_state
        .db_client
        .update_user_name(user_id.clone(), &body.name)
        .await
        .map_err(|e| MyHttpError::server_error(e.to_string()))?;

    let filtered_user = FilterUserDto::filter_user(&result);

    let response = UserResponseDto {
        data: UserData { user: filtered_user },
        status: "success".to_string(),
    };

    Ok(Json(response))
}
