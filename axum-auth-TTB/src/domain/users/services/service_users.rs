use axum::{
    middleware,
    routing::{get, put},
    Router,
};

use super::{
    get_me::get_me, get_users::get_users, update_user_name::update_user_name,
    update_user_password::update_user_password, update_user_role::update_user_role,
};
use crate::{middleware::role_check, models::UserRole};

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                        🦀 MAIN 🦀                          */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

pub fn service_users() -> Router {
    Router::new()
        .route(
            "/me",
            get(get_me).layer(middleware::from_fn(|state, req, next| {
                role_check(state, req, next, vec![UserRole::Admin, UserRole::User])
            })),
        )
        .route(
            "/users",
            get(get_users).layer(middleware::from_fn(|state, req, next| {
                role_check(state, req, next, vec![UserRole::Admin])
            })),
        )
        .route("/name", put(update_user_name))
        .route("/role", put(update_user_role))
        .route("/password", put(update_user_password))
}
