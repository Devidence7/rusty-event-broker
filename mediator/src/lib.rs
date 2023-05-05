pub trait Message {
    // Message name
    fn name(&self) -> &str;
}

pub trait Response {
    fn data(&self) -> &str;
}

pub trait MessageHandler {
    fn can_handle(&self, message: &dyn Message) -> bool;

    fn send(&self, message: &dyn Message) -> Option<Box<dyn Response>>;
}

pub struct Mediator {
    handlers: Vec<Box<dyn MessageHandler>>,
}

impl Mediator {
    pub fn new() -> Self {
        Mediator {
            handlers: Vec::new(),
        }
    }

    pub fn register_handler(&mut self, handler: Box<dyn MessageHandler>) {
        self.handlers.push(handler);
    }

    pub fn send(&self, message: &dyn Message) -> Option<Box<dyn Response>> {
        for handler in &self.handlers {
            if handler.can_handle(message) {
                return handler.send(message);
            }
        }

        None
    }
}
