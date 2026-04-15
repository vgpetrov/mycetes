use crate::requests::CreatePlaceRequest;

pub struct CreatePlaceCommand {
    pub name: String,
    pub user: i64,
    pub latitude: f64,
    pub longitude: f64,
}

impl From<CreatePlaceRequest> for CreatePlaceCommand {
    fn from(value: CreatePlaceRequest) -> Self {
        CreatePlaceCommand {
            name: value.name,
            user: value.user,
            latitude: value.latitude,
            longitude: value.longitude,
        }
    }
}