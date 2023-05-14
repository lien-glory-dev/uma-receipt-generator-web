use actix_web::{web, HttpRequest, Responder, ResponseError};

use crate::controller;
use crate::error::ApiError;

pub async fn not_found(request: HttpRequest) -> impl Responder {
    ApiError::EndpointNotFound {
        path: request.path().to_string(),
    }
    .error_response()
}

pub fn receipts(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/receipts").service(controller::receipt::insert));
}
