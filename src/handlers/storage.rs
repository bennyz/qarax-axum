use super::*;

use axum::extract::{Json, Path};

use models::storage as storage_model;
use models::storage::{NewStorage, Storage};

pub async fn list(
    Extension(env): Extension<Environment>,
) -> Result<ApiResponse<Vec<Storage>>, ServerError> {
    let storages = storage_model::list(env.db()).await.map_err(|e| {
        tracing::error!("Failed to list storages, error: {}", e);
        ServerError::InternalError
    })?;

    Ok(ApiResponse {
        data: storages,
        code: StatusCode::OK,
    })
}

pub async fn add(
    Extension(env): Extension<Environment>,
    Json(storage): Json<NewStorage>,
) -> Result<ApiResponse<Uuid>, ServerError> {
    let storage_id = storage_model::add(env.db(), &storage).await.map_err(|e| {
        tracing::error!("Can't add storage: {}", e);
        ServerError::InternalError
    })?;

    Ok(ApiResponse {
        data: storage_id,
        code: StatusCode::CREATED,
    })
}

pub async fn get(
    Extension(env): Extension<Environment>,
    Path(storage_id): Path<Uuid>,
) -> Result<ApiResponse<Storage>, ServerError> {
    let storage = storage_model::by_id(env.db(), storage_id)
        .await
        .map_err(|e| {
            tracing::error!("Can't find storage, error: {}", e);
            ServerError::InternalError
        })?;

    Ok(ApiResponse {
        data: storage,
        code: StatusCode::CREATED,
    })
}
