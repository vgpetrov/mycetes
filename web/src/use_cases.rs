use crate::commands::CreatePlaceCommand;
use repository::{Place, PlacesRepository};
use std::error::Error;
use std::sync::Arc;

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
    ) -> Result<Place, Box<dyn Error>> {
        let place = create_place_command.into();
        self.place_repository.save(place).await
    }
}

impl From<CreatePlaceCommand> for Place {
    fn from(value: CreatePlaceCommand) -> Self {
        Place {
            id: None,
            name: value.name,
            user_id: value.user,
            latitude: value.latitude,
            longitude: value.longitude,
            is_deleted: false,
        }
    }
}
