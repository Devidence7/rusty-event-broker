pub trait Message {
    fn name(&self) -> &str;
}

pub trait Response {
    fn data(&self) -> &str;
}

/// How to send a message. This is the interface that the Mediator uses to send messages.
/// The Mediator will call can_handle() to determine if the transport can handle the message.
/// TODO: Add publish() method
/// TODO: Add subscribe() method
/// TODO: Add set_priority() method
pub trait MessageTransport {
    fn can_handle(&self, message: &dyn Message) -> bool;

    fn send(&self, message: &dyn Message) -> Option<Box<dyn Response>>;

    fn publish(&self, _message: &dyn Message) {
        // Not implemented
    }
}

pub struct Mediator {
    transports: Vec<Box<dyn MessageTransport>>,
}

impl Mediator {
    pub fn new() -> Self {
        Mediator {
            transports: Vec::new(),
        }
    }

    pub fn register_transport(&mut self, transport: Box<dyn MessageTransport>) {
        self.transports.push(transport);
    }

    pub fn send(&self, message: &dyn Message) -> Option<Box<dyn Response>> {
        for transport in &self.transports {
            if transport.can_handle(message) {
                return transport.send(message);
            }
        }

        None
    }

    pub fn publish(&self, message: &dyn Message) {
        for transport in &self.transports {
            if transport.can_handle(message) {
                transport.publish(message);
            }
        }
    }
}
