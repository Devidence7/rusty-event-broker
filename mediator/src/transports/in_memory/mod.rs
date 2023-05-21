use std::collections::HashMap;
use std::sync::Arc;

use crate::{EntryTransport, ExitTransport, MessageName, RequestHandler};

pub struct InMemoryTransport {
    request_handlers: HashMap<String, Arc<dyn RequestHandler>>,
}

impl InMemoryTransport {
    pub fn new() -> Self {
        InMemoryTransport {
            request_handlers: HashMap::new(),
        }
    }
}

impl EntryTransport for InMemoryTransport {
    fn register_request_handler(&mut self, handler: Arc<dyn RequestHandler>) {
        self.request_handlers
            .insert(handler.message_name().to_string(), handler.to_owned());
    }
}

impl ExitTransport for InMemoryTransport {
    fn request(&self, request: Arc<dyn MessageName>) -> Result<Arc<dyn MessageName>, String> {
        let handler = self
            .request_handlers
            .get(request.message_name())
            .ok_or_else(|| format!("No transport found for message: {}", request.message_name()))?;

        handler.handle_request(request)
    }
}
