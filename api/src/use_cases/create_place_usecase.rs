use crate::commands::CreatePlaceCommand;

use std::error::Error;
use std::sync::Arc;
use domain::aggregates::place_aggregate::PlaceAggregate;
use domain::domain_event::DomainEvent;
use domain::repository::PlacesRepository;

pub struct CreatePlaceUseCase {
    place_repository: Arc<dyn PlacesRepository + Send + Sync>,
}

impl CreatePlaceUseCase {
    pub fn new(place_repository: Arc<dyn PlacesRepository + Send + Sync>) -> Self {
        CreatePlaceUseCase { place_repository }
    }

    pub async fn create_place(
        &self,
        create_place_command: CreatePlaceCommand,
    ) -> Result<domain::Place, Box<dyn Error>> {
        let place: domain::Place = create_place_command.into();

        let mut place_aggregate = PlaceAggregate::new();
        place_aggregate.validate_before_save(place.clone());

        let result = self.place_repository.save(place).await;
        self.publish_events(place_aggregate.pull_domain_events());

        result
    }

    fn publish_events(&self, events: Vec<DomainEvent>) {
        todo!("Create an event publisher")
    }
}

impl From<CreatePlaceCommand> for domain::Place {
    fn from(value: CreatePlaceCommand) -> Self {
        domain::Place {
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