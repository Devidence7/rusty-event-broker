use std::rc::Rc;

use crate::{EntryTransport, ExitTransport, Request, RequestHandler, Response};

pub struct Broker {
    exit_transports: Vec<Box<dyn ExitTransport>>,
    entry_transports: Vec<Box<dyn EntryTransport>>,
}

impl Broker {
    pub fn new() -> Self {
        Broker {
            exit_transports: Vec::new(),
            entry_transports: Vec::new(),
        }
    }

    pub fn register_exit_transport(&mut self, transport: Box<dyn ExitTransport>) {
        self.exit_transports.push(transport);
    }

    pub fn register_entry_transport(&mut self, transport: Box<dyn EntryTransport>) {
        self.entry_transports.push(transport);
    }

    pub fn request(&mut self, message: Rc<dyn Request>) -> Option<Rc<dyn Response>> {
        for transport in self.exit_transports.iter_mut() {
            if transport.can_handle_request(message.clone()) {
                return transport.request(message);
            }
        }

        None
    }

    pub fn register_request_handler(&mut self, handler: &dyn RequestHandler) {
        for transport in &mut self.entry_transports {
            transport.register_request_handler(handler);
        }
    }
}
