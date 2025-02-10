use axum::{
    response::IntoResponse,
    Extension, Json,
};
use chrono::{Duration, Utc};
use std::sync::Arc;
use validator::Validate;

use crate::{
    db::UserExt,
    domain::{ mail::mails::{send_forgot_password_email}},
    domain::auth::dtos::dto_password::ForgotPasswordRequestDto,
    utils::{my_errors::MyHttpError, my_response::MyResponse},
    AppState,
};




pub async fn forgot_password(
    Extension(app_state): Extension<Arc<AppState>>,
    Json(body): Json<ForgotPasswordRequestDto>,
) -> Result<impl IntoResponse, MyHttpError> {
    body.validate().map_err(|e| MyHttpError::bad_request(e.to_string()))?;

    let result = app_state
        .db_client
        .get_user(None, None, Some(&body.email), None)
        .await
        .map_err(|e| MyHttpError::server_error(e.to_string()))?;

    let user = result.ok_or(MyHttpError::bad_request("Email not found!".to_string()))?;

    let verification_token = uuid::Uuid::new_v4().to_string();
    let expires_at = Utc::now() + Duration::minutes(30);

    let user_id = uuid::Uuid::parse_str(&user.id.to_string()).unwrap();

    app_state
        .db_client
        .add_verifed_token(user_id, &verification_token, expires_at)
        .await
        .map_err(|e| MyHttpError::server_error(e.to_string()))?;

    let reset_link = format!("http://localhost:5173/reset-password?token={}", &verification_token);

    let email_sent = send_forgot_password_email(&user.email, &reset_link, &user.name).await;

    if let Err(e) = email_sent {
        eprintln!("Failed to send forgot password email: {}", e);
        return Err(MyHttpError::server_error("Failed to send email".to_string()));
    }

    let response = MyResponse {
        message: "Password reset link has been sent to your email.".to_string(),
        status: "success",
    };

    Ok(Json(response))
}
