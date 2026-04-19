use domain::Spot;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct SpotResponse {
    pub pub_id: String,
    pub name: String,
    pub user: i64,
    pub latitude: f64,
    pub longitude: f64,
}

impl From<&Spot> for SpotResponse {
    fn from(spot: &Spot) -> Self {
        SpotResponse {
            pub_id: spot.pub_id.clone(),
            name: spot.name.clone(),
            user: spot.user_id,
            latitude: spot.latitude,
            longitude: spot.longitude,
        }
    }
}