use axum::{response::IntoResponse, Extension, Json};
use std::sync::Arc;

use crate::{
    common::middleware::JWTAuthMiddeware,
    domain::users::dtos::{
        dto_filter_user::FilterUserDto,
        dto_user_responses::{UserData, UserResponseDto},
    },
    utils::my_errors::MyHttpError,
    AppState,
};

/*´:°•.°+.*•´.*:˚.°*.˚•´.°:°•.°•.*•´.*:˚.°*.˚•´.°:°•.°+.*•´.*:*/
/*                     ✨ FUNCTIONS ✨                        */
/*.•°:°.´+˚.*°.˚:*.´•*.+°.•°:´*.´•*.•°.•°:°.´:•˚°.*°.˚:*.´+°.•*/

pub async fn users_get_me(
    Extension(_app_state): Extension<Arc<AppState>>,
    Extension(user): Extension<JWTAuthMiddeware>,
) -> Result<impl IntoResponse, MyHttpError> {
    let filtered_user = FilterUserDto::filter_user(&user.user);

    let response_data = UserResponseDto {
        status: "success".to_string(),
        data: UserData { user: filtered_user },
    };

    Ok(Json(response_data))
}
