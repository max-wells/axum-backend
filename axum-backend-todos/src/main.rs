use axum::{error_handling::HandleErrorLayer, Router};
use constants::others::PORT_3000;
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use utils::{db::Db, handle_timeout_error::handle_timeout_error, setup_tracing::setup_tracing};

mod constants;
mod models;
mod services;
mod utils;

use crate::services::service_todos::service_todos;

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                        🦀 MAIN 🦀                          */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[tokio::main]
async fn main() {
    setup_tracing();

    let db = Db::default();

    // Compose the routes
    let app = Router::new()
        .merge(service_todos())
        // Add middleware to all routes
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(handle_timeout_error))
                .timeout(Duration::from_secs(10))
                .layer(TraceLayer::new_for_http())
                .into_inner(),
        )
        .with_state(db);

    let listener = tokio::net::TcpListener::bind(PORT_3000).await.unwrap();

    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
