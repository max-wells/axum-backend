use async_trait::async_trait;
use axum::body::Body;
use axum::extract::{FromRequestParts, State};
use axum::http::request::Parts;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use tower_cookies::{Cookie, Cookies};

use crate::common::context::Context;
use crate::common::error::MyError;
use crate::common::error::MyResult;
use crate::common::model_controller::ModelController;
use crate::midleware::AUTH_TOKEN;
use crate::utils::parse_token::parse_token;

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

pub async fn my_middleware_require_auth(
	context: MyResult<Context>,
	req: Request<Body>,
	next: Next,
) -> MyResult<Response> {
	println!(
		"->> {:<12} - middleware_require_auth - {context:?}",
		"MIDDLEWARE"
	);

	context?;

	Ok(next.run(req).await)
}

pub async fn my_middleware_context_resolver(
	_mc: State<ModelController>,
	cookies: Cookies,
	mut req: Request<Body>,
	next: Next,
) -> MyResult<Response> {
	println!("->> {:<12} - middleware_context_resolver", "MIDDLEWARE");

	let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

	// Compute Result<Ctx>.
	let result_ctx = match auth_token
		.ok_or(MyError::AuthFailNoAuthTokenCookie)
		.and_then(parse_token)
	{
		Ok((user_id, _exp, _sign)) => {
			// TODO: Token components validations.
			Ok(Context::new(user_id))
		}
		Err(e) => Err(e),
	};

	// Remove the cookie if something went wrong other than NoAuthTokenCookie.
	if result_ctx.is_err() && !matches!(result_ctx, Err(MyError::AuthFailNoAuthTokenCookie)) {
		cookies.remove(Cookie::from(AUTH_TOKEN))
	}

	// Store the ctx_result in the request extension.
	req.extensions_mut().insert(result_ctx);

	Ok(next.run(req).await)
}

//
//
// ? Understand this
#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Context {
	type Rejection = MyError;

	async fn from_request_parts(parts: &mut Parts, _state: &S) -> MyResult<Self> {
		println!("->> {:<12} - Ctx", "EXTRACTOR");

		parts
			.extensions
			.get::<MyResult<Context>>()
			.ok_or(MyError::AuthFailCtxNotInRequestExt)?
			.clone()
	}
}
