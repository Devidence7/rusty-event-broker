// Define the message types
trait Message: std::fmt::Debug {}

#[derive(Debug)]
struct PingMessage;

impl Message for PingMessage {}

#[derive(Debug)]
struct PongMessage;

impl Message for PongMessage {}

// Define the handler trait
trait MessageHandler {
    fn handle(&self, message: &dyn Message);
}

// Define the in-memory handler
struct InMemoryHandler;

impl MessageHandler for InMemoryHandler {
    fn handle(&self, message: &dyn Message) {
        println!("Handling message: {:?}", message);
    }
}

// Define the mediator
struct Mediator<'a> {
    handlers: Vec<Box<dyn MessageHandler + 'a>>,
}

impl<'a> Mediator<'a> {
    fn new() -> Self {
        Mediator {
            handlers: Vec::new(),
        }
    }

    fn add_handler<T>(&mut self, handler: T)
    where
        T: MessageHandler + 'a,
    {
        self.handlers.push(Box::new(handler));
    }

    fn send(&self, message: Box<dyn Message>) {
        for handler in &self.handlers {
            handler.handle(&*message);
        }
    }
}

fn main() {
    // Create a mediator
    let mut mediator = Mediator::new();

    // Register the in-memory handler
    let in_memory_handler = InMemoryHandler;
    mediator.add_handler(in_memory_handler);

    // Send messages
    let ping_message: Box<dyn Message> = Box::new(PingMessage);
    mediator.send(ping_message);

    let pong_message: Box<dyn Message> = Box::new(PongMessage);
    mediator.send(pong_message);
}
