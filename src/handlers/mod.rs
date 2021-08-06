use crate::env::Environment;

use axum::{body::Body, extract::Extension, prelude::*, response::IntoResponse, routing::BoxRoute};
use http::{Response, StatusCode};
use serde::Serialize;
use serde_json::json;
use thiserror::Error;
use uuid::Uuid;

mod ansible;
pub mod hosts;
mod models;
pub mod storage;

pub fn hosts() -> BoxRoute<Body> {
    route("/", get(hosts::list).post(hosts::add))
        .route("/:id", get(hosts::get))
        .route("/:id/install", post(hosts::install))
        .boxed()
}

pub fn storage() -> BoxRoute<Body> {
    route("/", get(storage::list).post(storage::add))
        .route("/:id", get(storage::get))
        .boxed()
}

pub struct ApiResponse<T> {
    data: T,
    code: StatusCode,
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Send + Sync + Serialize,
{
    fn into_response(self) -> http::Response<Body> {
        let mut response = response::Json(json!({
            "response": self.data,
        }))
        .into_response();

        *response.status_mut() = self.code;
        response
    }
}

#[derive(Error, Debug, Serialize)]
pub enum ServerError {
    #[error("Internal error")]
    #[serde(rename(serialize = "internal error"))]
    InternalError,
    #[error("Validation error")]
    #[serde(rename(serialize = "validation error"))]
    ValidationError(String),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response<Body> {
        let code = match self {
            ServerError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            ServerError::ValidationError(_) => StatusCode::CONFLICT,
        };

        let mut response = response::Json(json!({
            "error": self,
        }))
        .into_response();
        *response.status_mut() = code;

        response
    }
}
