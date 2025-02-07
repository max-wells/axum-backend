use axum::{error_handling::HandleErrorLayer, Router};
use std::time::Duration;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

mod constants;
mod models;
mod services;
mod utils;

use crate::constants::others::PORT_8000;
use crate::services::service_todos::service_todos;
use crate::utils::{
    db::DbArcRwLock, handle_timeout_error::handle_timeout_error, setup_tracing::setup_tracing,
};

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                        🦀 MAIN 🦀                          */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[tokio::main]
async fn main() {
    setup_tracing();

    let db = DbArcRwLock::default();

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

    let listener = TcpListener::bind(PORT_8000).await.unwrap();

    println!("🚀 Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
