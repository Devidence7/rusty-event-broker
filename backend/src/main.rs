trait Message {
    // Message name
    fn name(&self) -> &str;
}

struct A1Message {
    pub name: String,
    pub age: u8,
}

impl Message for A1Message {
    fn name(&self) -> &str {
        "A1Message"
    }
}

struct A2Message;

impl Message for A2Message {
    fn name(&self) -> &str {
        "A2Message"
    }
}

struct B1Message;

impl Message for B1Message {
    fn name(&self) -> &str {
        "B1Message"
    }
}

struct NoHadlerMessage;

impl Message for NoHadlerMessage {
    fn name(&self) -> &str {
        "NoHadlerMessage"
    }
}

/// Message handler
/// Knows how to handle a specific message by its name
trait MessageHandler {
    fn can_handle(&self, message: &dyn Message) -> bool;

    fn handle(&self, message: &dyn Message);
}

struct AHandler {
    messages_to_handle: Vec<String>,
}

impl MessageHandler for AHandler {
    fn can_handle(&self, message: &dyn Message) -> bool {
        self.messages_to_handle
            .contains(&message.name().to_string())
    }

    fn handle(&self, message: &dyn Message) {
        println!("AHandler handle {}", message.name());
    }
}

struct BHandler {
    messages_to_handle: Vec<String>,
}

impl MessageHandler for BHandler {
    fn can_handle(&self, message: &dyn Message) -> bool {
        self.messages_to_handle
            .contains(&message.name().to_string())
    }

    fn handle(&self, message: &dyn Message) {
        println!("BHandler handle {}", message.name());
    }
}

struct Mediator {
    handlers: Vec<Box<dyn MessageHandler>>,
}

impl Mediator {
    fn new() -> Self {
        Mediator {
            handlers: Vec::new(),
        }
    }

    fn register_handler(&mut self, handler: Box<dyn MessageHandler>) {
        self.handlers.push(handler);
    }

    fn handle_message(&self, message: &dyn Message) {
        for handler in &self.handlers {
            if handler.can_handle(message) {
                handler.handle(message);
                return; // Should be only one handler per message
            }
        }

        println!("No handler for {}", message.name());
    }
}

fn main() {
    let mut mediator = Mediator::new();

    mediator.register_handler(Box::new(AHandler {
        messages_to_handle: vec!["A1Message".to_string(), "A2Message".to_string()],
    }));

    mediator.register_handler(Box::new(BHandler {
        messages_to_handle: vec!["B1Message".to_string()],
    }));

    let a1_message = A1Message {
        name: "something".to_string(),
        age: 10,
    };

    let a2_message = A2Message;
    let b1_message = B1Message;
    let no_handler_message = NoHadlerMessage;

    mediator.handle_message(&a1_message);
    mediator.handle_message(&a2_message);
    mediator.handle_message(&b1_message);
    mediator.handle_message(&no_handler_message);
}
