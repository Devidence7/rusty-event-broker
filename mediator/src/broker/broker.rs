use crate::{EntryTransport, Event, ExitTransport, Response};

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

    pub fn send(&self, message: &dyn Event) -> Option<Box<dyn Response>> {
        for transport in &self.exit_transports {
            if transport.can_handle(message) {
                return transport.request(message);
            }
        }

        None
    }

    pub fn publish(&self, message: &dyn Event) {
        for transport in &self.exit_transports {
            if transport.can_handle(message) {
                transport.publish(message);
            }
        }
    }
}
