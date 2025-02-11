use axum::{
    routing::{get, post},
    Router,
};

use crate::domain::auth::services::forgot_password::forgot_password;
use crate::domain::auth::services::login::login;
use crate::domain::auth::services::register::register;
use crate::domain::auth::services::reset_password::reset_password;
use crate::domain::auth::services::verify_email::verify_email;

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                        🦀 MAIN 🦀                          */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

pub fn auth_services() -> Router {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/verify", get(verify_email))
        .route("/forgot-password", post(forgot_password))
        .route("/reset-password", post(reset_password))
}
