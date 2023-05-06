use crate::{Event, Response};

/// How to send a message. This is the interface that the Mediator uses to send messages.
/// The Mediator will call can_handle() to determine if the transport can handle the message.
/// TODO: Add publish() method
/// TODO: Add subscribe() method
/// TODO: Add set_priority() method
pub trait Transport {
    fn can_handle(&self, message: &dyn Event) -> bool;

    fn send(&self, message: &dyn Event) -> Option<Box<dyn Response>>;

    fn publish(&self, _message: &dyn Event) {
        // Not implemented
    }
}
