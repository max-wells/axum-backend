use std::sync::Arc;
use axum::{
    extract::Query,
    response::IntoResponse,
    Extension, Json,
};
use validator::Validate;

use crate::{
    common::db::UserExt, domain::users::dtos::dtos::{FilterUserDto, UserListResponseDto}, utils::{my_errors::MyHttpError, request_query_dto::RequestQueryDto}, AppState
};

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/





pub async fn users_get_all(
    Query(query_params): Query<RequestQueryDto>,
    Extension(app_state): Extension<Arc<AppState>>,
) -> Result<impl IntoResponse, MyHttpError> {
    query_params
        .validate()
        .map_err(|e| MyHttpError::bad_request(e.to_string()))?;

    let page = query_params.page.unwrap_or(1);
    let limit = query_params.limit.unwrap_or(10);

    let users = app_state
        .db_client
        .get_users(page as u32, limit)
        .await
        .map_err(|e| MyHttpError::server_error(e.to_string()))?;

    let user_count = app_state
        .db_client
        .get_user_count()
        .await
        .map_err(|e| MyHttpError::server_error(e.to_string()))?;

    let response = UserListResponseDto {
        status: "success".to_string(),
        users: FilterUserDto::filter_users(&users),
        results: user_count,
    };

    Ok(Json(response))
}
