use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

#[derive(Debug, PartialEq)]
pub enum MyErrorMessage {
    EmptyPassword,
    ExceededMaxPasswordLength(usize),
    InvalidHashFormat,
    HashingError,
    InvalidToken,
    ServerError,
    WrongCredentials,
    EmailExist,
    UserNoLongerExist,
    TokenNotProvided,
    PermissionDenied,
    UserNotAuthenticated,
}

impl ToString for MyErrorMessage {
    fn to_string(&self) -> String {
        self.to_str().to_owned()
    }
}

impl MyErrorMessage {
    fn to_str(&self) -> String {
        match self {
            MyErrorMessage::ServerError => "Server Error. Please try again later".to_string(),
            MyErrorMessage::WrongCredentials => "Email or password is wrong".to_string(),
            MyErrorMessage::EmailExist => "A user with this email already exists".to_string(),
            MyErrorMessage::UserNoLongerExist => "User belonging to this token no longer exists".to_string(),
            MyErrorMessage::EmptyPassword => "Password cannot be empty".to_string(),
            MyErrorMessage::HashingError => "Error while hashing password".to_string(),
            MyErrorMessage::InvalidHashFormat => "Invalid password hash format".to_string(),
            MyErrorMessage::ExceededMaxPasswordLength(max_length) => {
                format!("Password must not be more than {} characters", max_length)
            }
            MyErrorMessage::InvalidToken => "Authentication token is invalid or expired".to_string(),
            MyErrorMessage::TokenNotProvided => "You are not logged in, please provide a token".to_string(),
            MyErrorMessage::PermissionDenied => "You are not allowed to perform this action".to_string(),
            MyErrorMessage::UserNotAuthenticated => "Authentication required. Please log in.".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MyHttpError {
    pub message: String,
    pub status: StatusCode,
}

impl MyHttpError {
    pub fn new(message: impl Into<String>, status: StatusCode) -> Self {
        MyHttpError {
            message: message.into(),
            status,
        }
    }

    pub fn server_error(message: impl Into<String>) -> Self {
        MyHttpError {
            message: message.into(),
            status: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn bad_request(message: impl Into<String>) -> Self {
        MyHttpError {
            message: message.into(),
            status: StatusCode::BAD_REQUEST,
        }
    }

    pub fn unique_constraint_violation(message: impl Into<String>) -> Self {
        MyHttpError {
            message: message.into(),
            status: StatusCode::CONFLICT,
        }
    }

    pub fn unauthorized(message: impl Into<String>) -> Self {
        MyHttpError {
            message: message.into(),
            status: StatusCode::UNAUTHORIZED,
        }
    }

    pub fn into_http_response(self) -> Response {
        let json_response = Json(ErrorResponse {
            status: "fail".to_string(),
            message: self.message.clone(),
        });

        (self.status, json_response).into_response()
    }
}

impl fmt::Display for MyHttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HttpError: message: {}, status: {}", self.message, self.status)
    }
}

impl std::error::Error for MyHttpError {}

impl IntoResponse for MyHttpError {
    fn into_response(self) -> Response {
        self.into_http_response()
    }
}
