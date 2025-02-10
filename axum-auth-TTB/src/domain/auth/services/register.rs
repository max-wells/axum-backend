use axum::{
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use chrono::{Duration, Utc};
use std::sync::Arc;
use validator::Validate;

use crate::{
    db::UserExt,
    domain::{ auth::dtos::dto_register_user::RegisterUserDto, mail::mails::send_verification_email},
    utils::{my_errors::{MyErrorMessage, MyHttpError}, my_response::MyResponse, utils_password},
    AppState,
};


pub async fn register(
    Extension(app_state): Extension<Arc<AppState>>,
    Json(body): Json<RegisterUserDto>,
) -> Result<impl IntoResponse, MyHttpError> {
    body.validate().map_err(|e| MyHttpError::bad_request(e.to_string()))?;

    let verification_token = uuid::Uuid::new_v4().to_string();
    let expires_at = Utc::now() + Duration::hours(24);

    let hash_password = utils_password::hash(&body.password).map_err(|e| MyHttpError::server_error(e.to_string()))?;

    let result = app_state
        .db_client
        .save_user(&body.name, &body.email, &hash_password, &verification_token, expires_at)
        .await;

    match result {
        Ok(_user) => {
            let send_email_result = send_verification_email(&body.email, &body.name, &verification_token).await;

            if let Err(e) = send_email_result {
                eprintln!("Failed to send verification email: {}", e);
            }

            Ok((
                StatusCode::CREATED,
                Json(MyResponse {
                    status: "success",
                    message: "Registration successful! Please check your email to verify your account.".to_string(),
                }),
            ))
        }
        Err(sqlx::Error::Database(db_err)) => {
            if db_err.is_unique_violation() {
                Err(MyHttpError::unique_constraint_violation(
                    MyErrorMessage::EmailExist.to_string(),
                ))
            } else {
                Err(MyHttpError::server_error(db_err.to_string()))
            }
        }
        Err(e) => Err(MyHttpError::server_error(e.to_string())),
    }
}
