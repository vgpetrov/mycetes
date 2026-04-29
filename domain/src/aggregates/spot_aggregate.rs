use std::error::Error;
use crate::domain_event::{DomainEvent};
use crate::{Photo, Spot};

/// Represents a spot aggregate that holds a repository and a buffer of domain events.
///
/// This struct follows Domain-Driven Design conventions and allows
/// validating a `Spot` before saving, recording events, and pulling domain events.
pub struct SpotAggregate {
    spot: Spot,
    photo_vec: Vec<Photo>,
    event_buffer: Vec<DomainEvent>,
}

impl SpotAggregate {
    pub fn new(spot: Spot, photo_vec: Vec<Photo>) -> Self {
        SpotAggregate {
            spot,
            photo_vec,
            event_buffer: Vec::new(),
        }
    }

    pub fn validate_before_save(&mut self) -> Result<(), Box<dyn Error>> {
        // TODO: Moderate spot after save
        self.record_event(DomainEvent::SpotSavedEvent(self.spot.clone()));
        Ok(())
    }

    pub fn into_parts(self) -> (Vec<DomainEvent>, Spot, Vec<Photo>){
        (self.event_buffer, self.spot, self.photo_vec)
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