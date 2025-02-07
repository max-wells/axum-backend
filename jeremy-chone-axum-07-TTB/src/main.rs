use axum::middleware::{from_fn, from_fn_with_state, map_response};
use axum::Router;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;

mod common;
mod features;
mod midleware;
mod utils;

use crate::common::error::MyResult;
use crate::common::main_response_mapper::my_main_response_mapper;
use crate::common::model_controller::ModelController;
use crate::features::basics::routes_hello::routes_hello;
use crate::features::basics::routes_static::routes_static;
use crate::features::login::routes_login::routes_login;
use crate::features::tickets::routes_tickets::routes_tickets;
use crate::midleware::middleware_auth::my_middleware_context_resolver;
use crate::midleware::middleware_auth::my_middleware_require_auth;

// * cargo run

const PORT_8000: &str = "127.0.0.1:8000";

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                        🦀 MAIN 🦀                          */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[tokio::main]
async fn main() -> MyResult<()> {
	// ? Undserstand model_controller
	let model_controller = ModelController::new().await?;

	// ? Why my_middleware_require_auth() takes no arguments ? Should have 3 arguments
	// ? Is it because of the from_fn ?
	let routes_apis =
		routes_tickets(model_controller.clone()).route_layer(from_fn(my_middleware_require_auth));

	// ? from_fn_with_state()
	// ? nest()
	let routes_all = Router::new()
		.merge(routes_hello())
		.merge(routes_login())
		.nest("/api", routes_apis)
		.layer(map_response(my_main_response_mapper))
		.layer(from_fn_with_state(
			model_controller.clone(),
			my_middleware_context_resolver,
		))
		.layer(CookieManagerLayer::new())
		.fallback_service(routes_static());

	let listener = TcpListener::bind(PORT_8000).await.unwrap();
	println!("🚀 LISTENING on {:?}\n", listener.local_addr());

	// ? into_make_service()
	axum::serve(listener, routes_all.into_make_service())
		.await
		.unwrap();

	Ok(())
}
