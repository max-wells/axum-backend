use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing_subscriber::filter::LevelFilter;

mod common;
mod domain;
mod utils;

use common::app_state::AppState;
use crate::common::build_app_router::create_app_router;
use crate::common::config::Config;
use crate::common::db::DBClient;

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                        🦀 MAIN 🦀                          */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/


#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt().with_max_level(LevelFilter::DEBUG).init();

    let config = Config::init();

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
        .expect("🔥 Failed to connect to the database");

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE])
        .allow_credentials(true)
        .allow_methods([Method::GET, Method::POST, Method::PUT]);

    let db_client = DBClient::new(pool);
    let app_state = AppState {
        env: config.clone(),
        db_client,
    };

    let app_router = create_app_router(Arc::new(app_state.clone())).layer(cors.clone());

    println!("🚀 Server running on Port {}", config.port);

    let listener = TcpListener::bind(format!("0.0.0.0:{}", &config.port))
        .await
        .unwrap();

    axum::serve(listener, app_router).await.unwrap();
}
