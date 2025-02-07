use axum::http::{Method, Uri};
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;
use uuid::Uuid;

use super::ctx::Ctx;
use crate::common::error::MyError;
use crate::common::log::log_request;

pub async fn my_main_response_mapper(
	ctx: Option<Ctx>,
	uri: Uri,
	req_method: Method,
	res: Response,
) -> Response {
	println!("->> {:<12} - main_response_mapper", "RES_MAPPER");

	let uuid = Uuid::new_v4();

	// -- Get the eventual response error.
	let service_error = res.extensions().get::<MyError>();
	let client_status_error = service_error.map(|se| se.client_status_and_error());

	// -- If client error, build the new reponse.
	let error_response = client_status_error
		.as_ref()
		.map(|(status_code, client_error)| {
			let client_error_body = json!({
				"error": {
					"type": client_error.as_ref(),
					"req_uuid": uuid.to_string(),
				}
			});

			println!("    ->> client_error_body: {client_error_body}");

			// Build the new response from the client_error_body
			(*status_code, Json(client_error_body)).into_response()
		});

	// Build and log the server log line.
	let client_error = client_status_error.unzip().1;
	// TODO: Need to hander if log_request fail (but should not fail request)
	let _ = log_request(uuid, req_method, uri, ctx, service_error, client_error).await;

	println!();
	error_response.unwrap_or(res)
}
