use axum::{middleware, Router};
use common::model_controller::ModelController;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;

mod common;
mod features;
mod midleware;
mod utils;

use crate::common::main_response_mapper::main_response_mapper;
use crate::features::hello::hello_routes::{routes_hello, routes_static};
use crate::features::login::routes_login::routes_login;
use crate::features::tickets::routes_tickets::routes_tickets;

// TODO. Understand this
pub use self::common::error::{MyError, MyResult};

// * cargo run

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                        🦀 MAIN 🦀                          */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

#[tokio::main]
async fn main() -> MyResult<()> {
	// ? Undserstand model_controller
	let model_controller = ModelController::new().await?;

	// ? Understand why there are no arguments in middleware_require_auth function (should take 3)
	let routes_apis = routes_tickets(model_controller.clone()).route_layer(
		middleware::from_fn(midleware::middleware_auth::middleware_require_auth),
	);

	let routes_all = Router::new()
		.merge(routes_hello())
		.merge(routes_login())
		.nest("/api", routes_apis)
		.layer(middleware::map_response(main_response_mapper))
		.layer(middleware::from_fn_with_state(
			model_controller.clone(),
			midleware::middleware_auth::middleware_ctx_resolver,
		))
		.layer(CookieManagerLayer::new())
		.fallback_service(routes_static());

	let listener = TcpListener::bind("127.0.0.1:8000").await.unwrap();

	println!("->> LISTENING on {:?}\n", listener.local_addr());

	// ? Undserstand into_make_service
	axum::serve(listener, routes_all.into_make_service())
		.await
		.unwrap();

	Ok(())
}
