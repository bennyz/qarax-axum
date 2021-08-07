use super::*;

use axum::extract::{Json, Path};

use models::vms as vm_model;
use models::vms::{NewVm, Vm};

pub async fn list(
    Extension(env): Extension<Environment>,
) -> Result<ApiResponse<Vec<Vm>>, ServerError> {
    let vms = vm_model::list(env.db()).await.map_err(|e| {
        tracing::error!("Failed to list vms, error: {}", e);
        ServerError::Internal
    })?;

    Ok(ApiResponse {
        data: vms,
        code: StatusCode::OK,
    })
}

pub async fn add(
    Extension(env): Extension<Environment>,
    Json(vm): Json<NewVm>,
) -> Result<ApiResponse<Uuid>, ServerError> {
    let vm_id = vm_model::add(env.db(), &vm).await.map_err(|e| {
        tracing::error!("Can't add vm: {}", e);
        ServerError::Internal
    })?;

    Ok(ApiResponse {
        data: vm_id,
        code: StatusCode::CREATED,
    })
}

pub async fn get(
    Extension(env): Extension<Environment>,
    Path(vm_id): Path<Uuid>,
) -> Result<ApiResponse<Vm>, ServerError> {
    let vm = vm_model::by_id(env.db(), &vm_id).await.map_err(|e| {
        tracing::error!("Can't find vm, error: {}", e);
        ServerError::Internal
    })?;

    Ok(ApiResponse {
        data: vm,
        code: StatusCode::CREATED,
    })
}
