use crate::Spot;

pub enum DomainEvent {
    SpotSavedEvent(Spot),
    UserCreatedEvent(String),
}

impl DomainEvent {

    fn handle(&self) {
        match self {
            DomainEvent::SpotSavedEvent(spot) => {

            }
            DomainEvent::UserCreatedEvent(user) => {

            }
        }
    }
}