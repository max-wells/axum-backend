use axum::{
    extract::Query,
    http::{header, HeaderMap},
    response::{IntoResponse, Redirect},
    Extension,
};
use axum_extra::extract::cookie::Cookie;
use chrono::Utc;
use std::sync::Arc;
use validator::Validate;

use crate::{
    common::db::UserExt,
    domain::{auth::dtos::dto_verify_email_query::VerifyEmailQueryDto, mail::mails::send_welcome_email},
    utils::{my_errors::{MyErrorMessage, MyHttpError}, utils_token},
    AppState,
};



pub async fn verify_email(
    Query(query_params): Query<VerifyEmailQueryDto>,
    Extension(app_state): Extension<Arc<AppState>>,
) -> Result<impl IntoResponse, MyHttpError> {
    query_params
        .validate()
        .map_err(|e| MyHttpError::bad_request(e.to_string()))?;

    let result = app_state
        .db_client
        .get_user(None, None, None, Some(&query_params.token))
        .await
        .map_err(|e| MyHttpError::server_error(e.to_string()))?;

    let user = result.ok_or(MyHttpError::unauthorized(MyErrorMessage::InvalidToken.to_string()))?;

    if let Some(expires_at) = user.token_expires_at {
        if Utc::now() > expires_at {
            return Err(MyHttpError::bad_request("Verification token has expired".to_string()))?;
        }
    } else {
        return Err(MyHttpError::bad_request("Invalid verification token".to_string()))?;
    }

    app_state
        .db_client
        .verifed_token(&query_params.token)
        .await
        .map_err(|e| MyHttpError::server_error(e.to_string()))?;

    let send_welcome_email_result = send_welcome_email(&user.email, &user.name).await;

    if let Err(e) = send_welcome_email_result {
        eprintln!("Failed to send welcome email: {}", e);
    }

    let token = utils_token::create_token(
        &user.id.to_string(),
        app_state.env.jwt_secret.as_bytes(),
        app_state.env.jwt_maxage,
    )
    .map_err(|e| MyHttpError::server_error(e.to_string()))?;

    let cookie_duration = time::Duration::minutes(app_state.env.jwt_maxage * 60);
    let cookie = Cookie::build(("token", token.clone()))
        .path("/")
        .max_age(cookie_duration)
        .http_only(true)
        .build();

    let mut headers = HeaderMap::new();

    headers.append(header::SET_COOKIE, cookie.to_string().parse().unwrap());

    let frontend_url = format!("http://localhost:5173/settings");

    let redirect = Redirect::to(&frontend_url);

    let mut response = redirect.into_response();

    response.headers_mut().extend(headers);

    Ok(response)
}
