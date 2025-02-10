use axum::{
    http::{header, HeaderMap},
    response::IntoResponse,
    Extension, Json,
};
use axum_extra::extract::cookie::Cookie;
use std::sync::Arc;
use validator::Validate;

use crate::{
    db::UserExt,
    domain::auth::dtos::dto_login_user::LoginUserDto,
    domain::auth::dtos::dto_login_user::UserLoginResponseDto,
    error::{MyErrorMessage, MyHttpError},
    utils::{password, token},
    AppState,
};

pub async fn login(
    Extension(app_state): Extension<Arc<AppState>>,
    Json(body): Json<LoginUserDto>,
) -> Result<impl IntoResponse, MyHttpError> {
    body.validate().map_err(|e| MyHttpError::bad_request(e.to_string()))?;

    let result = app_state
        .db_client
        .get_user(None, None, Some(&body.email), None)
        .await
        .map_err(|e| MyHttpError::server_error(e.to_string()))?;

    let user = result.ok_or(MyHttpError::bad_request(MyErrorMessage::WrongCredentials.to_string()))?;

    let password_matched = password::compare(&body.password, &user.password)
        .map_err(|_| MyHttpError::bad_request(MyErrorMessage::WrongCredentials.to_string()))?;

    if password_matched {
        let token = token::create_token(
            &user.id.to_string(),
            &app_state.env.jwt_secret.as_bytes(),
            app_state.env.jwt_maxage,
        )
        .map_err(|e| MyHttpError::server_error(e.to_string()))?;

        let cookie_duration = time::Duration::minutes(app_state.env.jwt_maxage * 60);
        let cookie = Cookie::build(("token", token.clone()))
            .path("/")
            .max_age(cookie_duration)
            .http_only(true)
            .build();

        let response = axum::response::Json(UserLoginResponseDto {
            status: "success".to_string(),
            token,
        });

        let mut headers = HeaderMap::new();

        headers.append(header::SET_COOKIE, cookie.to_string().parse().unwrap());

        let mut response = response.into_response();
        response.headers_mut().extend(headers);

        Ok(response)
    } else {
        Err(MyHttpError::bad_request(MyErrorMessage::WrongCredentials.to_string()))
    }
}