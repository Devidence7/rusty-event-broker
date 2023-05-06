use crate::{Request, Response};
use std::rc::Rc;

/// Handler is an interface to provide a common way to handle requests (commands and queries).
pub trait RequestHandler {
    fn handle(&self, request: Rc<dyn Request>) -> Option<Rc<dyn Response>>;

    fn name(&self) -> &str;
}

/// EventHandler is an interface to provide a common way to handle events.
pub trait EventHandler {
    fn handle(&self, request: &dyn Request) -> &dyn Response;

    fn name(&self) -> &str;
}
