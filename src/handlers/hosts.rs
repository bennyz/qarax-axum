use super::*;
use models::hosts as host_model;
use models::hosts::Host;

pub async fn list(
    Extension(env): Extension<Environment>,
) -> Result<ApiResponse<Vec<Host>>, ServerError> {
    let hosts = host_model::list(env.db())
        .await
        .map_err(|_| ServerError::InternalError)?;

    Ok(ApiResponse {
        data: hosts,
        code: StatusCode::OK,
    })
}
