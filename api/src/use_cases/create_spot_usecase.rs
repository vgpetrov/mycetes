use std::error::Error;
use crate::commands::CreateSpotCommand;

use domain::aggregates::spot_aggregate::SpotAggregate;
use domain::domain_event::DomainEvent;
use domain::repository::SpotsRepository;
use std::sync::Arc;
use std::time::SystemTime;
use tracing::log;
use domain::file_storage::FileStorage;

#[derive(Debug)]
pub enum CreateSpotError {
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
        CreateSpotUseCase { spot_repository, file_storage }
    }

    pub async fn create_spot(
        &self,
        create_spot_command: CreateSpotCommand,
    ) -> Result<domain::Spot, CreateSpotError> {
        let spot: domain::Spot = create_spot_command.into();

        let mut spot_aggregate = SpotAggregate::new(spot);
        spot_aggregate
            .validate_before_save()
            .inspect_err(|e| log::error!("Failed to validate spot: {}", e))
            .map_err(|e| CreateSpotError::Validation(e.to_string()))?;

        let spot_aggregate_parts = spot_aggregate.into_parts();

        let result = self
            .spot_repository
            .save(spot_aggregate_parts.0)
            .await
            .map_err(|e| CreateSpotError::Db(e.to_string()))?;
        
        self.publish_events(spot_aggregate_parts.1);

        Ok(result)
    }

    fn publish_events(&self, events: Vec<DomainEvent>) {
        // todo!("Create an event publisher")
    }
}

impl From<CreateSpotCommand> for domain::Spot {
    fn from(value: CreateSpotCommand) -> Self {
        domain::Spot {
            pub_id: uuid::Uuid::new_v4().to_string(),
            name: value.name,
            user_id: value.user,
            latitude: value.latitude,
            longitude: value.longitude,
            created_at: SystemTime::now(),
            updated_at: SystemTime::now(),
            metadata: Option::from("".to_string()),
            approved_by: None,
            deleted: false,
        }
    }
}
