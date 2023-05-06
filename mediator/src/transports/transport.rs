use crate::{Event, Response};

pub trait ExitTransport {
    fn can_handle(&self, message: &dyn Event) -> bool;

    fn request(&self, message: &dyn Event) -> Option<Box<dyn Response>>;

    fn publish(&self, _message: &dyn Event) {
        // Not implemented
    }
}

pub trait EntryTransport {
    fn listen(&self);
}
