use crate::{Event, Request, Response};

pub trait EventHandler {
    fn handle(&self, event: &dyn Event);
}

pub trait RequestHandler {
    fn handle(&self, request: &dyn Request) -> Box<dyn Response>;
}
