use std::time::SystemTime;

pub mod domain_event;
pub mod repository;
pub mod stats;
pub mod aggregates;

pub struct User {
    pub name: String,
    pub email: String,
}

#[derive(Clone)]
pub struct Spot {
    pub pub_id: String,
    pub name: String,
    pub user_id: i64,
    pub latitude: f64,
    pub longitude: f64,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
    pub metadata: Option<String>,
    pub approved_by: Option<i64>,
    pub deleted: bool,
}
