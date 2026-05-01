use crate::repository::db::db_helper::DbHelper;
use crate::{SpotEntity, SpotLightEntity};
use anyhow::{anyhow};
use async_trait::async_trait;
use domain::Photo;
use domain::repository::SpotsRepository;
use sqlx::{Postgres, QueryBuilder};
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

        let result = spot.iter().map(|p| p.into()).collect();
        Ok(result)
    }

    async fn save(&self, mut spot: domain::Spot) -> Result<domain::Spot, Box<dyn Error>> {
        let pool = self.db_helper.get_pool().map_err(|e| {
            tracing::error!("Failed to get database pool: {:?}", e);
            anyhow!("Failed to connect to database")
        })?;

        sqlx::query_as::<_, SpotLightEntity>(
            r#"
                INSERT INTO spot (name, pub_id, user_id, latitude, longitude, metadata)
                VALUES ($1, $2, $3, $4, $5, $6)
                RETURNING id, pub_id
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
        .map_err(|e| {
            tracing::error!("Failed to insert spot into db: {:?}", e);
            anyhow!("Failed to insert spot")
        })?;

        Ok(spot)
    }

    async fn save_photos(
        &self,
        spot_pub_id: &str,
        photo_vec: Vec<Photo>,
    ) -> Result<(), Box<dyn Error>> {
        let pool = self.db_helper.get_pool().map_err(|e| {
            tracing::error!("Failed to get database pool: {:?}", e);
            anyhow!("Failed to connect to database")
        })?;

        let spot_id: i64 = sqlx::query_scalar(
            r#"
                SELECT id
                FROM spot
                WHERE pub_id = $1 AND deleted = false
                "#,
        )
        .bind(spot_pub_id)
        .fetch_one(pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to load spot: {:?}", e);
            anyhow!("Failed to load spot")
        })?;

        let mut builder = QueryBuilder::<Postgres>::new(
            r#"
            INSERT INTO spot_photo (spot_id, storage_key, metadata)
             "#,
        );

        builder.push_values(photo_vec, |mut b, photo| {
            let metadata = serde_json::json!({
                "fileName": photo.file_name,
                "contentType": photo.content_type,
            });
            b.push_bind(spot_id)
                .push_bind(photo.storage_key)
                .push_bind(metadata.to_string());
        });

        builder.build().execute(pool).await.map_err(|e| {
            tracing::error!("Failed to save photos: {:?}", e);
            anyhow!("Failed to save photos")
        })?;

        Ok(())
    }
}
