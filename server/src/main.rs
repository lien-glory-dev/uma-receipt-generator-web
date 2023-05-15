use std::io::Write;

use actix_files::Files;
use actix_web::{web, App, HttpRequest, HttpServer};

use error::ApiError;

mod controller;
mod error;
mod route;

const TEMP_UPLOAD_DIRECTORY: &str = "./images-temp";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::builder()
        .parse_default_env()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                chrono::Local::now().format("%F %T%.6f%:z"),
                record.level(),
                record.args()
            )
        })
        .init();

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

    HttpServer::new(move || {
        App::new()
            .app_data(
                actix_web_validator::QueryConfig::default().error_handler(request_error_handler),
            )
            .app_data(
                actix_web_validator::PathConfig::default().error_handler(request_error_handler),
            )
            .app_data(
                actix_web_validator::JsonConfig::default().error_handler(request_error_handler),
            )
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
            .default_service(web::route().to(route::not_found))
    })
    .worker_max_blocking_threads(1024)
    .bind(("0.0.0.0", 80))?
    .run()
    .await
}
