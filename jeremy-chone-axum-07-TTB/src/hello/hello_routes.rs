use axum::extract::{Path, Query};
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, get_service};
use axum::Router;
use serde::Deserialize;
use tower_http::services::ServeDir;

pub fn routes_static() -> Router {
	Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

// region:    --- Routes Hello
pub fn routes_hello() -> Router {
	Router::new()
		.route("/hello", get(handler_hello))
		.route("/hello2/:name", get(handler_hello2))
}

#[derive(Debug, Deserialize)]
pub struct HelloParams {
	pub name: Option<String>,
}

// e.g., `/hello?name=Jen`
pub async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
	println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");

	let name = params.name.as_deref().unwrap_or("World!");
	Html(format!("Hello <strong>{name}</strong>"))
}

// e.g., `/hello2/Mike`
pub async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
	println!("->> {:<12} - handler_hello2 - {name:?}", "HANDLER");

	Html(format!("Hello2 <strong>{name}</strong>"))
}

// endregion: --- Routes Hello
