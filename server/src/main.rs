use actix_files::Files;
use actix_web::{web, HttpRequest};
use shuttle_actix_web::ShuttleActixWeb;

use error::ApiError;

mod controller;
mod error;
mod route;

const TEMP_UPLOAD_DIRECTORY: &str = "./images-temp";

#[shuttle_runtime::main]
async fn actix_web(
) -> ShuttleActixWeb<impl FnOnce(&mut web::ServiceConfig) + Send + Clone + 'static> {
    log::info!("Creating temp upload dir");
    std::fs::create_dir_all(TEMP_UPLOAD_DIRECTORY)?;

    let request_error_handler =
        |err: actix_web_validator::Error, _req: &HttpRequest| -> actix_web::Error {
            {
                ApiError::InvalidParameter {
                    message: err.to_string().split(':').collect::<Vec<&str>>()[0].to_string(),
                    sensitive_message: Some(err.to_string()),
                }
                .into()
            }
        };

    let config = move |cfg: &mut web::ServiceConfig| {
        cfg.app_data(
            actix_web_validator::QueryConfig::default().error_handler(request_error_handler),
        )
        .app_data(actix_web_validator::PathConfig::default().error_handler(request_error_handler))
        .app_data(actix_web_validator::JsonConfig::default().error_handler(request_error_handler))
        .app_data(
            actix_web_validator::QsQueryConfig::default()
                .qs_config(serde_qs::Config::new(5, false))
                .error_handler(request_error_handler),
        )
        .app_data(
            actix_multipart::form::tempfile::TempFileConfig::default()
                .directory(TEMP_UPLOAD_DIRECTORY),
        )
        .configure(route::receipts)
        .service(Files::new("/", "./dist/").index_file("index.html"))
        .default_service(web::route().to(route::not_found));
    };

    Ok(config.into())
}
