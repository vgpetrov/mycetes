#[derive(serde::Serialize, serde::Deserialize)]
pub struct PlaceResponse {
    pub id: Option<i64>,
    pub name: String,
    pub user: i64,
    pub latitude: f64,
    pub longitude: f64,
}