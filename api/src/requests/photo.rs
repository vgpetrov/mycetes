use crate::app_errors::AppError;
use domain::Photo;
use image::imageops::FilterType;
use image::{GenericImageView, ImageFormat};
use std::io::Cursor;

const MAX_PHOTO_SIZE_BYTES: usize = 5 * 1024 * 1024; // 5 MB
const MAX_IMAGE_WIDTH: u32 = 10_000;
const MAX_IMAGE_HEIGHT: u32 = 10_000;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct PhotoRequest {
    pub file_name: String,
    pub content_type: String,
    pub thumbnail_bytes: Vec<u8>,
    pub bytes: Vec<u8>,
}

impl From<PhotoRequest> for Photo {
    fn from(value: PhotoRequest) -> Self {
        Photo {
            spot_pub_id: None,
            storage_key: None,
            file_name: value.file_name,
            content_type: value.content_type,
            thumbnail_bytes: Some(value.thumbnail_bytes),
            bytes: Some(value.bytes),
        }
    }
}

impl PhotoRequest {
    pub fn normalize_content_type(content_type: &str) -> Option<&str> {
        let content_type = content_type
            .split(';')
            .next()
            .unwrap_or(content_type)
            .trim()
            .to_ascii_lowercase();

        match content_type.as_str() {
            "image/jpeg" | "image/jpg" => Some("image/jpeg"),
            "image/png" => Some("image/png"),
            "image/webp" => Some("image/webp"),
            _ => None,
        }
    }

    pub fn content_type_from_image_format(format: ImageFormat) -> Option<&'static str> {
        match format {
            ImageFormat::Jpeg => Some("image/jpeg"),
            ImageFormat::Png => Some("image/png"),
            ImageFormat::WebP => Some("image/webp"),
            _ => None,
        }
    }

    pub fn validate_photo(
        file_name: String,
        declared_content_type: String,
        bytes: Vec<u8>,
    ) -> Result<(String, String, Vec<u8>), AppError> {
        if bytes.is_empty() {
            return Err(AppError::ValidationError("Photo is empty".to_string()));
        }

        if bytes.len() > MAX_PHOTO_SIZE_BYTES {
            return Err(AppError::ValidationError(
                "Photo is larger than 5 MB".to_string(),
            ));
        }

        let declared_content_type = PhotoRequest::normalize_content_type(&declared_content_type)
            .ok_or_else(|| {
                AppError::ValidationError("Unsupported photo content type".to_string())
            })?;

        let actual_format = image::guess_format(&bytes)
            .map_err(|_| AppError::ValidationError("File is not a supported image".to_string()))?;

        let actual_content_type = PhotoRequest::content_type_from_image_format(actual_format)
            .ok_or_else(|| AppError::ValidationError("Unsupported image format".to_string()))?;

        if declared_content_type != actual_content_type {
            return Err(AppError::ValidationError(format!(
                "Declared content type does not match actual file format: declared={}, actual={}",
                declared_content_type, actual_content_type
            )));
        }

        let image = image::load_from_memory_with_format(&bytes, actual_format)
            .map_err(|_| AppError::ValidationError("Invalid image file".to_string()))?;

        let (width, height) = image.dimensions();

        if width > MAX_IMAGE_WIDTH || height > MAX_IMAGE_HEIGHT {
            return Err(AppError::ValidationError(
                "Image dimensions are too large".to_string(),
            ));
        }

        Ok((file_name, actual_content_type.to_string(), bytes))
    }

    pub fn create_thumbnail(bytes: &[u8]) -> Result<Vec<u8>, image::ImageError> {
        let img = image::load_from_memory(bytes)?;

        let thumb = img.resize(768, 768, FilterType::Lanczos3);

        let mut output = Cursor::new(Vec::new());

        thumb.write_to(&mut output, image::ImageFormat::Jpeg)?;

        Ok(output.into_inner())
    }
}
