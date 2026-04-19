use crate::SpotEntity;
use crate::db::db_helper::DbHelper;
use async_trait::async_trait;
use domain::repository::SpotsRepository;
use std::error::Error;
use std::sync::Arc;
use anyhow::Context;
use tracing::log;

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

        let spot = sqlx::query_as::<_, SpotEntity>(
            r#"
                SELECT id, name, user_id, latitude, longitude, deleted
                FROM spot
                WHERE deleted = false
                    AND approved_by IS NOT NULL
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

    async fn save(&self, mut spot: domain::Spot) -> Result<domain::Spot, Box<dyn Error>> {
        let pool = self.db_helper.get_pool()?;

        let spot_entity = sqlx::query_as::<_, SpotEntity>(
            r#"
                INSERT INTO spot (name, pub_id, user_id, latitude, longitude, metadata)
                VALUES ($1, $2, $3, $4, $5, $6)
                RETURNING id, pub_id, name, user_id, latitude, longitude, deleted, created_at, updated_at, metadata
            "#,
        )
            .bind(&spot.name)
            .bind(&spot.pub_id)
            .bind(&spot.user_id)
            .bind(&spot.latitude)
            .bind(&spot.longitude)
            .bind("{}")
            .fetch_one(pool)
            .await
            // .context("Failed to insert spot")?;
            .inspect_err(|e| log::error!("Failed to insert spot 2: {}", e))
            ?;

        Ok(spot)
    }
}
