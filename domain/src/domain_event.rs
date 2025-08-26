
pub trait DomainEvent {
}

pub struct PlaceSavedEvent {
}

impl PlaceSavedEvent {
    pub fn new() -> Self {
        PlaceSavedEvent {}
    }
}

impl DomainEvent for PlaceSavedEvent {
}

pub struct UserCreatedEvent {
}

impl UserCreatedEvent {
    pub fn new() -> Self {
        UserCreatedEvent {}
    }
}

impl DomainEvent for UserCreatedEvent {
}