pub mod domain_event;
pub mod repository;
pub mod stats;
pub mod aggregates;

pub struct User {
    pub name: String,
    pub email: String,
}

#[derive(Clone)]
pub struct Place {
    pub id: Option<i64>,
    pub name: String,
    pub user_id: i64,
    pub latitude: f64,
    pub longitude: f64,
    pub metadata: String,
    pub is_deleted: bool,
}
