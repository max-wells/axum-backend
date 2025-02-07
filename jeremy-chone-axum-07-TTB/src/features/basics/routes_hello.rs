use axum::extract::{Path, Query};
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::Router;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct HelloParams {
	pub name: Option<String>,
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                        🦀 MAIN 🦀                          */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

pub fn routes_hello() -> Router {
	Router::new()
		.route("/hello", get(hello_with_query))
		.route("/hello2/:name", get(hello_with_path))
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

// 1. Hello with path
pub async fn hello_with_path(Path(name): Path<String>) -> impl IntoResponse {
	println!("->> {:<12} - hello_with_path - {name:?}", "HANDLER");

	Html(format!("Hello2 <strong>{name}</strong>"))
}

// 2. Hello with query
pub async fn hello_with_query(Query(params): Query<HelloParams>) -> impl IntoResponse {
	println!("->> {:<12} - hello_with_query - {params:?}", "HANDLER");

	let name = params.name.as_deref().unwrap_or("World!");
	Html(format!("Hello <strong>{name}</strong>"))
}
