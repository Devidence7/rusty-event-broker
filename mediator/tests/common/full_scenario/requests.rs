use mediator::{Event, Response};

pub struct A1Message {
    pub name: String,
    pub age: u8,
}

impl Event for A1Message {
    fn name(&self) -> &str {
        "A1Message"
    }
}

pub struct A2Message;

impl Event for A2Message {
    fn name(&self) -> &str {
        "A2Message"
    }
}

pub struct B1Message;

impl Event for B1Message {
    fn name(&self) -> &str {
        "B1Message"
    }
}

pub struct NoTransportMessage;

impl Event for NoTransportMessage {
    fn name(&self) -> &str {
        "NoTransportMessage"
    }
}

pub struct GenericResponse {
    pub data: String,
}

impl Response for GenericResponse {
    fn data(&self) -> &str {
        &self.data
    }

    fn name(&self) -> &str {
        "GenericResponse"
    }
}
