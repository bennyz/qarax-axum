use std::collections::BTreeMap;

use crate::handlers::ansible::AnsibleCommand;
use crate::handlers::models::hosts::HostError;

use super::models::hosts::{NewHost, Status};
use super::*;
use axum::extract::{Json, Path};
use models::hosts as host_model;
use models::hosts::Host;

pub async fn list(
    Extension(env): Extension<Environment>,
) -> Result<ApiResponse<Vec<Host>>, ServerError> {
    tracing::info!("list works");
    let hosts = host_model::list(env.db()).await.map_err(|e| {
        tracing::error!("Failed to list hosts, error: {}", e);
        ServerError::Internal
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
        ServerError::Internal
    })?;

    Ok(ApiResponse {
        data: host_id,
        code: StatusCode::CREATED,
    })
}

pub async fn get(
    Extension(env): Extension<Environment>,
    Path(host_id): Path<Uuid>,
) -> Result<ApiResponse<Host>, ServerError> {
    let host = host_model::by_id(env.db(), &host_id).await.map_err(|e| {
        tracing::error!("Failed to find host: {}, error:{}", host_id, e);

        match e {
            HostError::Find(id, sqlx::Error::RowNotFound) => {
                ServerError::EntityNotFound(id.to_string())
            }
            _ => ServerError::Internal,
        }
    })?;

    Ok(ApiResponse {
        data: host,
        code: StatusCode::CREATED,
    })
}

pub async fn install(
    Extension(env): Extension<Environment>,
    Path(host_id): Path<Uuid>,
) -> Result<ApiResponse<String>, ServerError> {
    let host = if let Ok(host) = host_model::by_id(env.db(), &host_id).await {
        host
    } else {
        tracing::error!("Failed to find host: {}", host_id);
        return Err(ServerError::Internal);
    };

    host_model::update_status(env.db(), host_id, Status::Installing)
        .await
        .map_err(|e| {
            tracing::error!("Failed to update host: {}, error:{}", host_id, e);
            ServerError::Internal
        })?;

    let mut extra_params = BTreeMap::new();
    extra_params.insert(String::from("ansible_password"), host.password.to_owned());

    // TODO: find a better place for the version
    extra_params.insert(String::from("fcversion"), String::from("0.24.5"));

    tokio::spawn(async move {
        let playbook = AnsibleCommand::new(
            ansible::INSTALL_HOST_PLAYBOOK,
            &host.host_user,
            &host.address,
            extra_params,
        );

        match playbook.run_playbook().await {
            Ok(_) => {
                tracing::info!("Installation successful");
                host_model::update_status(env.db(), host_id, Status::Up)
                    .await
                    .unwrap();
            }

            Err(e) => tracing::error!("Installation failed: {}", e),
        }
    });

    Ok(ApiResponse {
        code: StatusCode::ACCEPTED,
        data: String::from("started"),
    })
}
