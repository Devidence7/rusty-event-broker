use mediator::{Mediator, Message, MessageHandler, Response};

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

struct GenericResponse {
    pub data: String,
}

impl Response for GenericResponse {
    fn data(&self) -> &str {
        &self.data
    }
}

struct AHandler {
    messages_to_handle: Vec<String>,
}

impl MessageHandler for AHandler {
    fn can_handle(&self, message: &dyn Message) -> bool {
        self.messages_to_handle
            .contains(&message.name().to_string())
    }

    fn send(&self, _message: &dyn Message) -> Option<Box<dyn Response>> {
        return Some(Box::new(GenericResponse {
            data: "AHandler response".to_string(),
        }));
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

    fn send(&self, _message: &dyn Message) -> Option<Box<dyn Response>> {
        return Some(Box::new(GenericResponse {
            data: "BHandler response".to_string(),
        }));
    }
}

#[test]
fn check_send_multiple_handlers() {
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

    let a1_response = mediator.send(&a1_message);
    assert!(a1_response.is_some());
    assert_eq!(a1_response.unwrap().data(), "AHandler response");

    let a2_response = mediator.send(&a2_message);
    assert!(a2_response.is_some());
    assert_eq!(a2_response.unwrap().data(), "AHandler response");

    let b1_response = mediator.send(&b1_message);
    assert!(b1_response.is_some());
    assert_eq!(b1_response.unwrap().data(), "BHandler response");

    let no_handler_response = mediator.send(&no_handler_message);
    assert!(no_handler_response.is_none());
}
