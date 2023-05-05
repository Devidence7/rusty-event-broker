use mediator::{Mediator, Message, MessageTransport, Response};

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

struct NoTransportMessage;

impl Message for NoTransportMessage {
    fn name(&self) -> &str {
        "NoTransportMessage"
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

struct ATransport {
    messages_to_handle: Vec<String>,
}

impl MessageTransport for ATransport {
    fn can_handle(&self, message: &dyn Message) -> bool {
        self.messages_to_handle
            .contains(&message.name().to_string())
    }

    fn send(&self, _message: &dyn Message) -> Option<Box<dyn Response>> {
        return Some(Box::new(GenericResponse {
            data: "Transport A response".to_string(),
        }));
    }
}

struct BTransport {
    messages_to_handle: Vec<String>,
}

impl MessageTransport for BTransport {
    fn can_handle(&self, message: &dyn Message) -> bool {
        self.messages_to_handle
            .contains(&message.name().to_string())
    }

    fn send(&self, _message: &dyn Message) -> Option<Box<dyn Response>> {
        return Some(Box::new(GenericResponse {
            data: "Transport B response".to_string(),
        }));
    }
}

#[test]
fn check_send_multiple_transports() {
    let mut mediator = Mediator::new();

    mediator.register_transport(Box::new(ATransport {
        messages_to_handle: vec!["A1Message".to_string(), "A2Message".to_string()],
    }));

    mediator.register_transport(Box::new(BTransport {
        messages_to_handle: vec!["B1Message".to_string()],
    }));

    let a1_message = A1Message {
        name: "something".to_string(),
        age: 10,
    };

    let a2_message = A2Message;
    let b1_message = B1Message;
    let no_transport_message = NoTransportMessage;

    let a1_response = mediator.send(&a1_message);
    assert!(a1_response.is_some());
    assert_eq!(a1_response.unwrap().data(), "Transport A response");

    let a2_response = mediator.send(&a2_message);
    assert!(a2_response.is_some());
    assert_eq!(a2_response.unwrap().data(), "Transport A response");

    let b1_response = mediator.send(&b1_message);
    assert!(b1_response.is_some());
    assert_eq!(b1_response.unwrap().data(), "Transport B response");

    let no_transport_response = mediator.send(&no_transport_message);
    assert!(no_transport_response.is_none());
}
