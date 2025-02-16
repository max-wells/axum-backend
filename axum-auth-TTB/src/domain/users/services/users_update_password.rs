use axum::{response::IntoResponse, Extension, Json};
use std::sync::Arc;
use validator::Validate;

use crate::{
    common::{db::UserExt, middleware::JWTAuthMiddeware},
    domain::users::dtos::dto_update_user_password::UserPasswordUpdateDto,
    utils::{
        my_errors::{MyErrorMessage, MyHttpError},
        my_response::MyResponse,
        utils_password,
    },
    AppState,
};

pub async fn users_update_password(
    Extension(app_state): Extension<Arc<AppState>>,
    Extension(user): Extension<JWTAuthMiddeware>,
    Json(body): Json<UserPasswordUpdateDto>,
) -> Result<impl IntoResponse, MyHttpError> {
    body.validate().map_err(|e| MyHttpError::bad_request(e.to_string()))?;

    let user = &user.user;

    let user_id = uuid::Uuid::parse_str(&user.id.to_string()).unwrap();

    let result = app_state
        .db_client
        .get_user(Some(user_id.clone()), None, None, None)
        .await
        .map_err(|e| MyHttpError::server_error(e.to_string()))?;

    let user = result.ok_or(MyHttpError::unauthorized(MyErrorMessage::InvalidToken.to_string()))?;

    let password_match = utils_password::compare(&body.old_password, &user.password)
        .map_err(|e| MyHttpError::server_error(e.to_string()))?;

    if !password_match {
        return Err(MyHttpError::bad_request("Old password is incorrect".to_string()));
    }

    let hash_password =
        utils_password::hash(&body.new_password).map_err(|e| MyHttpError::server_error(e.to_string()))?;

    app_state
        .db_client
        .update_user_password(user_id.clone(), hash_password)
        .await
        .map_err(|e| MyHttpError::server_error(e.to_string()))?;

    let response = MyResponse {
        message: "Password updated Successfully".to_string(),
        status: "success",
    };

    Ok(Json(response))
}
