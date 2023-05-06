use crate::{Event, Response, Transport};

pub struct Mediator {
    transports: Vec<Box<dyn Transport>>,
}

impl Mediator {
    pub fn new() -> Self {
        Mediator {
            transports: Vec::new(),
        }
    }

    pub fn register_transport(&mut self, transport: Box<dyn Transport>) {
        self.transports.push(transport);
    }

    pub fn send(&self, message: &dyn Event) -> Option<Box<dyn Response>> {
        for transport in &self.transports {
            if transport.can_handle(message) {
                return transport.send(message);
            }
        }

        None
    }

    pub fn publish(&self, message: &dyn Event) {
        for transport in &self.transports {
            if transport.can_handle(message) {
                transport.publish(message);
            }
        }
    }
}
