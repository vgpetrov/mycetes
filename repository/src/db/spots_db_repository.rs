use crate::Spot;
use crate::db::db_helper::DbHelper;
use async_trait::async_trait;
use domain::repository::SpotsRepository;
use std::error::Error;
use std::sync::Arc;

pub struct SpotsDbRepository {
    db_helper: Arc<DbHelper>,
}

impl SpotsDbRepository {
    pub fn new(db_helper: Arc<DbHelper>) -> Self {
        Self { db_helper }
    }
}

#[async_trait]
impl SpotsRepository for SpotsDbRepository {
    async fn list_spots(&self) -> Result<Vec<domain::Spot>, Box<dyn Error>> {
        let pool = self.db_helper.get_pool()?;

        let spot = sqlx::query_as::<_, Spot>(
            r#"
                SELECT id, name, user_id, latitude, longitude, is_deleted
                FROM spot
                WHERE is_deleted = false
                ORDER BY id
                "#,
        )
        .fetch_all(pool)
        .await?;

        let result = spot.iter()
            .map(|p| p.into())
            .collect();
        Ok(result)
    }

    async fn save(&self, spot: domain::Spot) -> Result<domain::Spot, Box<dyn Error>> {
        let pool = self.db_helper.get_pool()?;

        let spot = sqlx::query_as::<_, Spot>(
            r#"
                INSERT INTO spot (name, user_id, latitude, longitude)
                VALUES ($1, $2, $3, $4)
                RETURNING id, name, user_id, latitude, longitude, is_deleted
            "#,
        )
            .bind(spot.name)
            .bind(spot.user_id)
            .bind(spot.latitude)
            .bind(spot.longitude)
            .fetch_one(pool)
            .await?;

        Ok(spot.into())
    }
}
