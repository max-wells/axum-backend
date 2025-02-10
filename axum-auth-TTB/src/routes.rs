use std::sync::Arc;

use axum::{middleware, Extension, Router};
use tower_http::trace::TraceLayer;

use crate::{
    domain::{auth::services::auth_services::auth_services, users::users_handler::users_handler},
    middleware::auth,
    AppState,
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    let api_route = Router::new()
        .nest("/auth", auth_services())
        .nest("/users", users_handler().layer(middleware::from_fn(auth)))
        .layer(TraceLayer::new_for_http())
        .layer(Extension(app_state));

    Router::new().nest("/api", api_route)
}
