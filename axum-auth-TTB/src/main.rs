use std::sync::Arc;
use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};
use common::app_state::AppState;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::CorsLayer;
use tracing_subscriber::filter::LevelFilter;

mod domain;
mod utils;
mod common;


use crate::common::build_app_router::create_app_router;
use crate::common::config::Config;
use crate::common::db::DBClient;


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_max_level(LevelFilter::DEBUG).init();

    dotenv().ok();

    let config = Config::init();

    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
    {
        Ok(pool) => {
            println!("✅Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

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

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", &config.port))
        .await
        .unwrap();

    axum::serve(listener, app_router).await.unwrap();
}
