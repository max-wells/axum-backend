use std::sync::Arc;
use axum::{
    response::IntoResponse,
    Extension, Json,
};

use crate::{
    domain::users::dtos::dtos::{FilterUserDto, UserData, UserResponseDto},
    error::MyHttpError,
    middleware::JWTAuthMiddeware,
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
