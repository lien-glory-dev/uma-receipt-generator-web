use actix_web::{error, HttpResponse, ResponseError};
use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use derive_more::{Display};
use log::error;
use thiserror::Error;
use serde::Serialize;

#[derive(Debug, Display, Error, Serialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum ApiError {
    EndpointNotFound {
        path: String,
    },
    ResourceNotFound {
        id: String,
    },
    #[display(fmt = "Invalid Parameter: {}", message)]
    InvalidParameter {
        message: String,
        #[serde(skip)]
        sensitive_message: Option<String>,
    },
    #[serde(rename = "image_upload_error")]
    IoError {
        #[serde(skip)]
        #[from]
        source: std::io::Error
    },
    ImageUploadError {
        message: String,
    },
    ImageGenerateError {
        message: String,
    },
    ImageProcessFailed {
        #[serde(skip)]
        #[from]
        source: uma_details_utility::image::Error
    }
}

#[derive(Debug, Display, Serialize)]
struct ApiErrorContainer<'a> {
    error: &'a ApiError,
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::EndpointNotFound { .. } | ApiError::ResourceNotFound { .. } => {
                StatusCode::NOT_FOUND
            }
            ApiError::InvalidParameter { .. } => StatusCode::BAD_REQUEST,
            ApiError::IoError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::ImageUploadError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::ImageGenerateError { .. } => StatusCode::BAD_REQUEST,
            ApiError::ImageProcessFailed { .. } => StatusCode::BAD_REQUEST,
        }
    }
    
    fn error_response(&self) -> HttpResponse<BoxBody> {
        error!("Responded error: {:?}", self);
        HttpResponse::build(self.status_code()).json(ApiErrorContainer { error: self })
    }
}
