use std::fs;
use std::io::Cursor;

use actix_multipart::form::{tempfile::TempFile, text::Text, MultipartForm};
use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use actix_web::{post, HttpRequest, HttpResponse, Responder, ResponseError};
use image::ImageOutputFormat;
use log::{error, info};
use uma_details_utility::image::detail::{HeaderTrimMode, HorseGirlFullDetailImage, ImageConfig};
use uma_details_utility::image::ImageMatrix;

use crate::error::ApiError;
use crate::TEMP_UPLOAD_DIRECTORY;

#[derive(Debug, MultipartForm)]
pub struct CreateReceiptRequest {
    trim_margin: Option<Text<i32>>,
    trim_close_button: Option<Text<i32>>,
    trim_title: Option<Text<i32>>,
    #[multipart(rename = "images[]")]
    images: Vec<TempFile>,
}

pub struct ReceiptCreatedResponse {
    image: image::DynamicImage,
}

impl Responder for ReceiptCreatedResponse {
    type Body = BoxBody;

    fn respond_to(self, _: &HttpRequest) -> HttpResponse<Self::Body> {
        let mut bytes = Cursor::new(Vec::new());
        self.image
            .write_to(&mut bytes, ImageOutputFormat::Png)
            .map(|_| {
                info!("Responded ok");

                HttpResponse::Ok()
                    .insert_header(ContentType::png())
                    .body(bytes.into_inner())
            })
            .unwrap_or_else(|_| {
                ApiError::ImageGenerateError {
                    message: "Failed to generate image".to_string(),
                }
                .error_response()
            })
    }
}

#[post("")]
pub async fn insert(
    MultipartForm(request): MultipartForm<CreateReceiptRequest>,
) -> Result<ReceiptCreatedResponse, ApiError> {
    let request_id = uuid::Uuid::new_v4();

    let trim_margin = request.trim_margin.map_or(Default::default(), |i| i.0) != 0;
    let trim_close_button = request.trim_close_button.map_or(Default::default(), |i| i.0) != 0;
    let trim_title = request.trim_title.map_or(Default::default(), |i| i.0) != 0;

    let dir_path = format!("{}/{}", TEMP_UPLOAD_DIRECTORY, request_id);
    let lock_path = format!("{}/.lock", dir_path);

    let lock_file = fs::read(lock_path.clone());
    if lock_file.is_ok() {
        error!("Directory {} is locked!", dir_path);
        return Err(ApiError::ImageUploadError {
            message: "Failed to upload image".to_string(),
        });
    }

    fs::create_dir_all(dir_path.clone())?;
    fs::write(lock_path, "")?;

    let mut index = 1;
    for image in request.images {
        let mime = image
            .content_type
            .clone()
            .ok_or(ApiError::InvalidParameter {
                message: "Invalid image".to_string(),
                sensitive_message: Some("Cannot identifying file content type".to_string()),
            })?;

        if mime != mime::IMAGE_PNG {
            return Err(ApiError::InvalidParameter {
                message: "Unsupported file type".to_string(),
                sensitive_message: Some(format!("File type {} is not supported", mime)),
            });
        }

        let file_path = format!("{}/{}.png", dir_path, index);

        fs::create_dir_all(dir_path.clone())?;

        image
            .file
            .persist(file_path.as_str())
            .map_err(|_| ApiError::ImageUploadError {
                message: "Failed to upload image".to_string(),
            })?;
        info!("Image uploaded to {:?}", file_path);

        index += 1;
    }

    let header_trim_mode = if trim_margin {
        Some(if trim_title {
            HeaderTrimMode::TrimTitleBar
        } else {
            HeaderTrimMode::TrimMarginOnly
        })
    } else {
        None
    };

    let detail = HorseGirlFullDetailImage::from_path(
        dir_path.as_str(),
        10,
        ImageConfig {
            do_merge_close_button: !trim_close_button,
            header_trim_mode,
            scaling_threshold_pixels: Some(540000)
        },
    )?;

    for file in fs::read_dir(dir_path.clone())? {
        fs::remove_file(file?.path())?;
    }
    fs::remove_dir(dir_path)?;

    let response = detail
        .convert_to_image()
        .map(|image| ReceiptCreatedResponse { image })?;

    Ok(response)
}
