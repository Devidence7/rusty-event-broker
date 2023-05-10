use std::rc::Rc;

use crate::{Request, RequestHandler, Response};

pub trait ExitTransport {
    fn can_handle_request(&self, message: Rc<dyn Request>) -> bool;

    fn request(&mut self, message: Rc<dyn Request>) -> Option<Rc<dyn Response>>;
}

pub trait EntryTransport {
    fn listen(&self);

    fn register_request_handler(&mut self, handler: Rc<dyn RequestHandler>);
}
