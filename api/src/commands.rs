use crate::requests::CreateSpotRequest;

pub struct CreateSpotCommand {
    pub name: String,
    pub user: i64,
    pub latitude: f64,
    pub longitude: f64,
}

impl From<CreateSpotRequest> for CreateSpotCommand {
    fn from(value: CreateSpotRequest) -> Self {
        CreateSpotCommand {
            name: value.name,
            user: value.user,
            latitude: value.latitude,
            longitude: value.longitude,
        }
    }
}