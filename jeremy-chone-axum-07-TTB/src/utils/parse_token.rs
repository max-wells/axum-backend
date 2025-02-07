use lazy_regex::regex_captures;

use crate::common::error::MyError;
use crate::common::error::MyResult;

/// Parse a token of format `user-[user-id].[expiration].[signature]`
/// Returns (user_id, expiration, signature)
pub fn parse_token(token: String) -> MyResult<(u64, String, String)> {
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
