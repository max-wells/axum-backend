use axum::{
    middleware,
    routing::{get, put},
    Router,
};

use super::{
    users_get_all::users_get_all, users_get_me::users_get_me, users_update_name::users_update_name,
    users_update_password::users_update_password, users_update_role::users_update_role,
};
use crate::{common::middleware::role_check, domain::users::models::models_user::UserRole};

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                        🦀 MAIN 🦀                          */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

pub fn service_users() -> Router {
    Router::new()
        .route(
            "/me",
            get(users_get_me).layer(middleware::from_fn(|state, req, next| {
                role_check(state, req, next, vec![UserRole::Admin, UserRole::User])
            })),
        )
        .route(
            "/users",
            get(users_get_all).layer(middleware::from_fn(|state, req, next| {
                role_check(state, req, next, vec![UserRole::Admin])
            })),
        )
        .route("/name", put(users_update_name))
        .route("/role", put(users_update_role))
        .route("/password", put(users_update_password))
}
