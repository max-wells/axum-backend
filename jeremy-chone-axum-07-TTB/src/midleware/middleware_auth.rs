use async_trait::async_trait;
use axum::body::Body;
use axum::extract::{FromRequestParts, State};
use axum::http::request::Parts;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use lazy_regex::regex_captures;
use tower_cookies::{Cookie, Cookies};

use crate::common::ctx::Ctx;
use crate::common::error::MyError;
use crate::common::error::MyResult;
use crate::common::model_controller::ModelController;
use crate::midleware::AUTH_TOKEN;

pub async fn middleware_require_auth(
	ctx: MyResult<Ctx>,
	req: Request<Body>,
	next: Next,
) -> MyResult<Response> {
	println!("->> {:<12} - mw_require_auth - {ctx:?}", "MIDDLEWARE");

	ctx?;

	Ok(next.run(req).await)
}

pub async fn middleware_ctx_resolver(
	_mc: State<ModelController>,
	cookies: Cookies,
	mut req: Request<Body>,
	next: Next,
) -> MyResult<Response> {
	println!("->> {:<12} - mw_ctx_resolver", "MIDDLEWARE");

	let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

	// Compute Result<Ctx>.
	let result_ctx = match auth_token
		.ok_or(MyError::AuthFailNoAuthTokenCookie)
		.and_then(parse_token)
	{
		Ok((user_id, _exp, _sign)) => {
			// TODO: Token components validations.
			Ok(Ctx::new(user_id))
		}
		Err(e) => Err(e),
	};

	// Remove the cookie if something went wrong other than NoAuthTokenCookie.
	if result_ctx.is_err()
		&& !matches!(result_ctx, Err(MyError::AuthFailNoAuthTokenCookie))
	{
		cookies.remove(Cookie::from(AUTH_TOKEN))
	}

	// Store the ctx_result in the request extension.
	req.extensions_mut().insert(result_ctx);

	Ok(next.run(req).await)
}

// region:    --- Ctx Extractor
#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
	type Rejection = MyError;

	async fn from_request_parts(parts: &mut Parts, _state: &S) -> MyResult<Self> {
		println!("->> {:<12} - Ctx", "EXTRACTOR");

		parts
			.extensions
			.get::<MyResult<Ctx>>()
			.ok_or(MyError::AuthFailCtxNotInRequestExt)?
			.clone()
	}
}

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

// endregion: --- Ctx Extractor

/// Parse a token of format `user-[user-id].[expiration].[signature]`
/// Returns (user_id, expiration, signature)
fn parse_token(token: String) -> MyResult<(u64, String, String)> {
	let (_whole, user_id, exp, sign) = regex_captures!(
		r#"^user-(\d+)\.(.+)\.(.+)"#, // a literal regex
		&token
	)
	.ok_or(MyError::AuthFailTokenWrongFormat)?;

	let user_id: u64 = user_id
		.parse()
		.map_err(|_| MyError::AuthFailTokenWrongFormat)?;

	Ok((user_id, exp.to_string(), sign.to_string()))
}
