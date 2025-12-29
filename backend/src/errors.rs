use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use serde_json::json;
use std::fmt;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Country not found: {0}")]
    CountryNotFound(String),

    #[error("Indicator not found: {0}")]
    IndicatorNotFound(String),

    #[error("No data available")]
    NoDataAvailable,

    #[error("External API error: {0}")]
    ExternalApiError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Internal server error")]
    InternalError,
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::CountryNotFound(_) | ApiError::IndicatorNotFound(_) => {
                StatusCode::NOT_FOUND
            }
            ApiError::NoDataAvailable => StatusCode::NOT_FOUND,
            ApiError::ExternalApiError(_) | ApiError::SerializationError(_) => {
                StatusCode::BAD_GATEWAY
            }
            ApiError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status = self.status_code();
        let error_response = json!({
            "error": status.to_string(),
            "message": self.to_string()
        });

        HttpResponse::build(status).json(error_response)
    }
}

impl From<reqwest::Error> for ApiError {
    fn from(err: reqwest::Error) -> Self {
        ApiError::ExternalApiError(err.to_string())
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(err: serde_json::Error) -> Self {
        ApiError::SerializationError(err.to_string())
    }
}
