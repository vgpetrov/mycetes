#[derive(serde::Serialize, serde::Deserialize)]
pub struct SpotResponse {
    pub id: Option<i64>,
    pub name: String,
    pub user: i64,
    pub latitude: f64,
    pub longitude: f64,
}

impl From<&domain::Spot> for SpotResponse {
    fn from(spot: &domain::Spot) -> Self {
        SpotResponse {
            id: spot.id,
            name: spot.name.clone(),
            user: spot.user_id,
            latitude: spot.latitude,
            longitude: spot.longitude,
        }
    }
}