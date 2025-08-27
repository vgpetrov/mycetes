use crate::commands::CreatePlaceCommand;
use std::error::Error;
use std::sync::Arc;
use domain::repository::PlacesRepository;

pub struct CreatePlaceUseCase {
    place_repository: Arc<Box<dyn PlacesRepository + Send + Sync>>,
}

impl CreatePlaceUseCase {
    pub fn new(place_repository: Arc<Box<dyn PlacesRepository + Send + Sync>>) -> Self {
        CreatePlaceUseCase { place_repository }
    }

    pub async fn create_place(
        &self,
        create_place_command: CreatePlaceCommand,
    ) -> Result<domain::Place, Box<dyn Error>> {
        let place = create_place_command.into();
        self.place_repository.save(place).await
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
