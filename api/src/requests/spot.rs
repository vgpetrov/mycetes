use domain::Photo;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CreateSpotRequest {
    pub name: String,
    pub user: i64,
    pub latitude: f64,
    pub longitude: f64,
    pub metadata: Option<String>,
    pub photos: Vec<PhotoRequest>
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct PhotoRequest {
    pub file_name: String,
    pub content_type: String,
    pub bytes: Vec<u8>,
}

// impl From<Photo> for PhotoRequest {
//     fn from(value: Photo) -> Self {
//         PhotoRequest {
//             file_name: value.file_name,
//             content_type: value.content_type,
//             bytes: value.bytes.unwrap(),
//         }
//     }
// }

impl From<PhotoRequest> for Photo {
    fn from(value: PhotoRequest) -> Self {
        Photo {
            spot_pub_id: None,
            storage_key: None,
            file_name: value.file_name,
            content_type: value.content_type,
            bytes: Some(value.bytes),
        }
    }
}