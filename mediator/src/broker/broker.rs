use std::{rc::Rc, sync::Mutex};

use crate::{EntryTransport, ExitTransport, Request, RequestHandler, Response};

pub struct Broker {
    exit_transports: Vec<Rc<Mutex<dyn ExitTransport>>>,
    entry_transports: Vec<Rc<Mutex<dyn EntryTransport>>>,
}

impl Broker {
    pub fn new() -> Self {
        Broker {
            exit_transports: Vec::new(),
            entry_transports: Vec::new(),
        }
    }

    pub fn register_exit_transport(&mut self, transport: Rc<Mutex<dyn ExitTransport>>) {
        self.exit_transports.push(transport);
    }

    pub fn register_entry_transport(&mut self, transport: Rc<Mutex<dyn EntryTransport>>) {
        self.entry_transports.push(transport);
    }

    pub fn request(&mut self, message: Rc<dyn Request>) -> Option<Rc<dyn Response>> {
        for transport in self.exit_transports.iter_mut() {
            let mut borrowed_transport = transport.lock().unwrap(); // TODO: improve this
            if borrowed_transport.can_handle_request(message.clone()) {
                return borrowed_transport.request(message);
            }
        }

        None
    }

    pub fn register_request_handler(&mut self, handler: Rc<dyn RequestHandler>) {
        for transport in &mut self.entry_transports {
            let mut borrowed_transport = transport.lock().unwrap(); // TODO: improve this
            borrowed_transport.register_request_handler(handler.clone());
        }
    }
}
