use axum::{
    response::IntoResponse,
    Extension, Json,
};
use chrono::Utc;
use std::sync::Arc;
use validator::Validate;

use crate::{
    db::UserExt,
    domain::auth::dtos::dto_password::ResetPasswordRequestDto,
    utils::my_errors::MyHttpError,
    utils::{my_response::MyResponse, utils_password},
    AppState,
};





pub async fn reset_password(
    Extension(app_state): Extension<Arc<AppState>>,
    Json(body): Json<ResetPasswordRequestDto>,
) -> Result<impl IntoResponse, MyHttpError> {
    body.validate().map_err(|e| MyHttpError::bad_request(e.to_string()))?;

    let result = app_state
        .db_client
        .get_user(None, None, None, Some(&body.token))
        .await
        .map_err(|e| MyHttpError::server_error(e.to_string()))?;

    let user = result.ok_or(MyHttpError::bad_request("Invalid or expired token".to_string()))?;

    if let Some(expires_at) = user.token_expires_at {
        if Utc::now() > expires_at {
            return Err(MyHttpError::bad_request("Verification token has expired".to_string()))?;
        }
    } else {
        return Err(MyHttpError::bad_request("Invalid verification token".to_string()))?;
    }

    let user_id = uuid::Uuid::parse_str(&user.id.to_string()).unwrap();

    let hash_password = utils_password::hash(&body.new_password).map_err(|e| MyHttpError::server_error(e.to_string()))?;

    app_state
        .db_client
        .update_user_password(user_id.clone(), hash_password)
        .await
        .map_err(|e| MyHttpError::server_error(e.to_string()))?;

    app_state
        .db_client
        .verifed_token(&body.token)
        .await
        .map_err(|e| MyHttpError::server_error(e.to_string()))?;

    let response = MyResponse {
        message: "Password has been successfully reset.".to_string(),
        status: "success",
    };

    Ok(Json(response))
}
