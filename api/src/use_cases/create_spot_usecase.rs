use crate::commands::{CreateSpotCommand};

use std::error::Error;
use std::sync::Arc;
use domain::aggregates::spot_aggregate::SpotAggregate;
use domain::domain_event::DomainEvent;
use domain::repository::SpotsRepository;

pub struct CreateSpotUseCase {
    spot_repository: Arc<dyn SpotsRepository + Send + Sync>,
}

impl CreateSpotUseCase {
    pub fn new(spot_repository: Arc<dyn SpotsRepository + Send + Sync>) -> Self {
        CreateSpotUseCase { spot_repository }
    }

    pub async fn create_spot(
        &self,
        create_spot_command: CreateSpotCommand,
    ) -> Result<domain::Spot, Box<dyn Error>> {
        let spot: domain::Spot = create_spot_command.into();

        let mut spot_aggregate = SpotAggregate::new();
        spot_aggregate.validate_before_save(spot.clone());

        let result = self.spot_repository.save(spot).await;
        self.publish_events(spot_aggregate.pull_domain_events());

        result
    }

    fn publish_events(&self, events: Vec<DomainEvent>) {
        // todo!("Create an event publisher")
    }
}

impl From<CreateSpotCommand> for domain::Spot {
    fn from(value: CreateSpotCommand) -> Self {
        domain::Spot {
            id: None,
            name: value.name,
            user_id: value.user,
            latitude: value.latitude,
            longitude: value.longitude,
            metadata: "".to_string(),
            is_deleted: false,
        }
    }
}