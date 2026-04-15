
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CreatePlaceRequest {
    pub name: String,
    pub user: i64,
    pub latitude: f64,
    pub longitude: f64,
}