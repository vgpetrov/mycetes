use crate::db::db_helper::DbHelper;
use crate::{Place, PlacesRepository};
use async_trait::async_trait;
use std::error::Error;
use std::sync::Arc;

pub struct PlacesDbRepository {
    db_helper: Arc<DbHelper>,
}

impl PlacesDbRepository {
    pub fn new(db_helper: Arc<DbHelper>) -> Self {
        Self { db_helper }
    }
}

#[async_trait]
impl PlacesRepository for PlacesDbRepository {
    async fn list_places(&self) -> Result<Vec<Place>, Box<dyn Error>> {
        let pool = self.db_helper.get_pool()?;

        let places = sqlx::query_as::<_, Place>(
            r#"
            SELECT id, name, user_id, latitude, longitude, is_deleted
            FROM place
            WHERE is_deleted = false
            ORDER BY id
            "#,
        )
        .fetch_all(pool)
        .await?;

        Ok(places)
    }

    async fn save(&self, place: Place) -> Result<Place, Box<dyn Error>> {
        let pool = self.db_helper.get_pool()?;

        let place = sqlx::query_as::<_, Place>(
            r#"
            INSERT INTO place (name, user_id, latitude, longitude)
            VALUES ($1, $2, $3, $4)
            RETURNING id, name, user_id, latitude, longitude, is_deleted
        "#,
        )
        .bind(place.name)
        .bind(place.user_id)
        .bind(place.latitude)
        .bind(place.longitude)
        .fetch_one(pool)
        .await?;

        Ok(place)
    }
}
