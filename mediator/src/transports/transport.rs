use std::sync::Arc;

use crate::{MessageName, RequestHandler};

pub trait ExitTransport {
    fn request(&self, request: Arc<dyn MessageName>) -> Result<Arc<dyn MessageName>, String>;
}

pub trait EntryTransport {
    fn register_request_handler(&mut self, handler: Arc<dyn RequestHandler>);
}
