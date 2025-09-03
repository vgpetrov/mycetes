use std::sync::Arc;
use crate::domain_event::{DomainEvent};
use crate::Place;
use crate::repository::PlacesRepository;

/// Represents a place aggregate that holds a repository and a buffer of domain events.
///
/// This struct follows Domain-Driven Design conventions and allows
/// validating a `Place` before saving, recording events, and pulling domain events.
pub struct PlaceAggregate {
    place_repository: Arc<dyn PlacesRepository + Send + Sync>,
    event_buffer: Vec<DomainEvent>,
}

impl PlaceAggregate {
    pub fn new(place_repository: Arc<dyn PlacesRepository + Send + Sync>) -> Self {
        PlaceAggregate {
            place_repository,
            event_buffer: Vec::new(),
        }
    }

    pub fn validate_before_save(&mut self, place: Place) {
        self.record_event(DomainEvent::PlaceSavedEvent(place))
    }

    fn record_event(&mut self, event: DomainEvent) {
        &self.event_buffer.push(event);
    }

    ///
    /// Returns a vector of domain events and moves the ownership
    ///
    pub fn pull_domain_events(self) -> Vec<DomainEvent> {
        self.event_buffer
    }
}