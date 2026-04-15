use crate::domain_event::{DomainEvent};
use crate::Spot;

/// Represents a spot aggregate that holds a repository and a buffer of domain events.
///
/// This struct follows Domain-Driven Design conventions and allows
/// validating a `Spot` before saving, recording events, and pulling domain events.
pub struct SpotAggregate {
    event_buffer: Vec<DomainEvent>,
}

impl SpotAggregate {
    pub fn new() -> Self {
        SpotAggregate {
            event_buffer: Vec::new(),
        }
    }

    pub fn validate_before_save(&mut self, spot: Spot) {
        self.record_event(DomainEvent::SpotSavedEvent(spot))
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