
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CreateSpotRequest {
    pub name: String,
    pub user: i64,
    pub latitude: f64,
    pub longitude: f64,
}