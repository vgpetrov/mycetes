use std::time::SystemTime;
use domain::Photo;
use crate::requests::CreateSpotRequest;

pub struct CreateSpotCommand {
    pub name: String,
    pub user: i64,
    pub latitude: f64,
    pub longitude: f64,
    pub metadata: Option<String>,
    pub photos: Vec<Photo>,
}

impl From<CreateSpotRequest> for CreateSpotCommand {
    fn from(value: CreateSpotRequest) -> CreateSpotCommand {
        CreateSpotCommand {
            name: value.name,
            user: value.user,
            latitude: value.latitude,
            longitude: value.longitude,
            metadata: None,
            photos: value.photos.into_iter().map(|photo| photo.into()).collect(),
        }
    }
}

impl From<CreateSpotCommand> for domain::Spot {
    fn from(value: CreateSpotCommand) -> Self {
        domain::Spot {
            pub_id: uuid::Uuid::new_v4().to_string(),
            name: value.name,
            user_id: value.user,
            latitude: value.latitude,
            longitude: value.longitude,
            created_at: SystemTime::now(),
            updated_at: SystemTime::now(),
            metadata: Option::from("{}".to_string()),
            approved_by: None,
            deleted: false,
        }
    }
}

impl From<CreateSpotCommand> for (domain::Spot, Vec<Photo>) {
    fn from(value: CreateSpotCommand) -> (domain::Spot, Vec<Photo>) {
        (domain::Spot {
            pub_id: uuid::Uuid::new_v4().to_string(),
            name: value.name,
            user_id: value.user,
            latitude: value.latitude,
            longitude: value.longitude,
            created_at: SystemTime::now(),
            updated_at: SystemTime::now(),
            metadata: Option::from("{}".to_string()),
            approved_by: None,
            deleted: false,
        }, value.photos)
    }
}