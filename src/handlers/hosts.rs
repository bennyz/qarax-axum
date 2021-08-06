use super::models::hosts::NewHost;
use super::*;
use axum::extract::Json;
use models::hosts as host_model;
use models::hosts::Host;

pub async fn list(
    Extension(env): Extension<Environment>,
) -> Result<ApiResponse<Vec<Host>>, ServerError> {
    let hosts = host_model::list(env.db()).await.map_err(|e| {
        tracing::error!("Failed to add host {}", e);
        ServerError::InternalError
    })?;

    Ok(ApiResponse {
        data: hosts,
        code: StatusCode::OK,
    })
}

pub async fn add(
    Extension(env): Extension<Environment>,
    Json(host): Json<NewHost>,
) -> Result<ApiResponse<Uuid>, ServerError> {
    let host_id = host_model::add(env.db(), &host).await.map_err(|e| {
        tracing::error!("Failed to add host {}", e);
        ServerError::InternalError
    })?;

    Ok(ApiResponse {
        data: host_id,
        code: StatusCode::CREATED,
    })
}
