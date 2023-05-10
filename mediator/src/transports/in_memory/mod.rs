use std::rc::Rc;

use crate::{EntryTransport, ExitTransport, Request, RequestHandler, Response};

pub struct InMemoryTransport {
    message_queue: std::collections::VecDeque<Rc<dyn Request>>,
    request_handlers: std::collections::HashMap<String, Rc<dyn RequestHandler>>,
}

impl InMemoryTransport {
    pub fn new() -> InMemoryTransport {
        return InMemoryTransport {
            message_queue: std::collections::VecDeque::new(),
            request_handlers: std::collections::HashMap::new(),
        };
    }
}

impl ExitTransport for InMemoryTransport {
    fn can_handle_request(&self, message: Rc<dyn Request>) -> bool {
        let handler = self.request_handlers.get(message.name());

        return handler.is_some();
    }

    fn request(&mut self, message: Rc<dyn Request>) -> Option<Rc<dyn Response>> {
        let handler = self.request_handlers.get(message.name());

        if handler.is_none() {
            return None;
        }

        let handler = handler.unwrap();

        let response = handler.handle(message);

        return response;
    }
}

impl EntryTransport for InMemoryTransport {
    fn listen(&self) {
        todo!()
    }

    fn register_request_handler(&mut self, handler: Rc<dyn RequestHandler>) {
        self.request_handlers
            .insert(handler.name().to_string(), handler);
    }
}
