use crate::commands::CreateSpotCommand;
use std::error::Error;

use domain::aggregates::spot_aggregate::SpotAggregate;
use domain::domain_event::DomainEvent;
use domain::file_storage::FileStorage;
use domain::repository::SpotsRepository;
use std::sync::Arc;
use tracing::log;
use domain::Photo;

#[derive(Debug)]
pub enum CreateSpotError {
    FileStorage(String),
    Db(String),
    Validation(String),
}

pub struct CreateSpotUseCase {
    spot_repository: Arc<dyn SpotsRepository + Send + Sync>,
    file_storage: Arc<dyn FileStorage + Send + Sync>,
}

impl CreateSpotUseCase {
    pub fn new(
        spot_repository: Arc<dyn SpotsRepository + Send + Sync>,
        file_storage: Arc<dyn FileStorage + Send + Sync>,
    ) -> Self {
        CreateSpotUseCase {
            spot_repository,
            file_storage,
        }
    }

    pub async fn create_spot(
        &self,
        create_spot_command: CreateSpotCommand,
    ) -> Result<domain::Spot, CreateSpotError> {
        let spot: (domain::Spot, Vec<Photo>)  = create_spot_command.into();

        let mut spot_aggregate = SpotAggregate::new(spot.0, spot.1);
        spot_aggregate
            .validate_before_save()
            .inspect_err(|e| log::error!("Failed to validate spot: {}", e))
            .map_err(|e| CreateSpotError::Validation(e.to_string()))?;

        let spot_aggregate_parts = spot_aggregate.into_parts();

        // TOOD: make steps them tokio concurrent
        // 1. Create Spot in DB
        let result = self
            .spot_repository
            .save(spot_aggregate_parts.1)
            .await
            .map_err(|e| CreateSpotError::Db(e.to_string()))?;

        // 2. Save photos in S3
        let mut photo_keys_vec: Vec<Photo> = Vec::new();
        for photo in spot_aggregate_parts.2 {
            let photo_key = self.file_storage
                .store(&result.pub_id, photo)
                .await
                .map_err(|e| CreateSpotError::FileStorage(e.to_string()))?;
            photo_keys_vec.push(photo_key);
        }

        // 3. Save links to photos in DB
        self.spot_repository
            .save_photos(&result.pub_id, photo_keys_vec)
            .await
            .map_err(|e| CreateSpotError::Db(e.to_string()))?;

        self.publish_events(spot_aggregate_parts.0);

        Ok(result)
    }

    fn publish_events(&self, events: Vec<DomainEvent>) {
        // todo!("Create an event publisher")
    }
}
