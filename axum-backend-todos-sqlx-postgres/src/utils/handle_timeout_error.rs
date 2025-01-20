use axum::http::StatusCode;
use tower::BoxError;

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ MIDDLEWARE ✨                         */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

pub async fn handle_timeout_error(error: BoxError) -> Result<StatusCode, (StatusCode, String)> {
    if error.is::<tower::timeout::error::Elapsed>() {
        Ok(StatusCode::REQUEST_TIMEOUT)
    } else {
        Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Unhandled internal error: {error}"),
        ))
    }
}
