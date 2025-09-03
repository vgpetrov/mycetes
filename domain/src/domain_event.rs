use crate::Place;

pub enum DomainEvent {
    PlaceSavedEvent(Place),
    UserCreatedEvent(String),
}

impl DomainEvent {

    fn handle(&self) {
        match self {
            DomainEvent::PlaceSavedEvent(place) => {

            }
            DomainEvent::UserCreatedEvent(user) => {

            }
        }
    }
}