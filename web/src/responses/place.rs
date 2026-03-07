#[derive(serde::Serialize, serde::Deserialize)]
pub struct PlaceResponse {
    pub id: Option<i64>,
    pub name: String,
    pub user: i64,
    pub latitude: f64,
    pub longitude: f64,
}

impl From<&domain::Place> for PlaceResponse {
    fn from(place: &domain::Place) -> Self {
        PlaceResponse {
            id: place.id,
            name: place.name.clone(),
            user: place.user_id,
            latitude: place.latitude,
            longitude: place.longitude,
        }
    }
}