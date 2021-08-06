use crate::env::Environment;

use axum::{body::Body, extract::Extension, prelude::*, response::IntoResponse, routing::BoxRoute};
use http::{Response, StatusCode};
use serde::Serialize;
use serde_json::json;
use thiserror::Error;
use uuid::Uuid;

pub mod hosts;
mod models;

pub fn hosts() -> BoxRoute<Body> {
    route("/", get(hosts::list).post(hosts::add)).boxed()
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
    ValidationError,
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response<Body> {
        let code = match self {
            ServerError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            ServerError::ValidationError => StatusCode::CONFLICT,
        };

        let mut response = response::Json(json!({
            "error": self,
        }))
        .into_response();
        *response.status_mut() = code;

        response
    }
}
