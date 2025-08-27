use sqlx::FromRow;

pub mod db;
pub mod mem;

#[derive(Debug, FromRow, Clone)]
pub struct Place {
    pub id: Option<i64>,
    pub name: String,
    pub user_id: i64,
    pub latitude: f64,
    pub longitude: f64,
    pub is_deleted: bool,
}

impl From<&Place> for domain::Place {
    fn from(value: &Place) -> Self {
        domain::Place {
            id: value.id,
            name: value.clone().name,
            user_id: value.user_id,
            latitude: value.latitude,
            longitude: value.longitude,
            metadata: String::from(""),
            is_deleted: value.is_deleted
        }
    }
}

impl From<Place> for domain::Place {
    fn from(value: Place) -> Self {
        domain::Place {
            id: value.id,
            name: value.clone().name,
            user_id: value.user_id,
            latitude: value.latitude,
            longitude: value.longitude,
            metadata: String::from(""),
            is_deleted: value.is_deleted
        }
    }
}

impl From<domain::Place> for Place {
    fn from(value: domain::Place) -> Self {
        Place {
            id: value.id,
            name: value.name,
            user_id: value.user_id,
            latitude: value.latitude,
            longitude: value.longitude,
            is_deleted: value.is_deleted,
        }
    }
}