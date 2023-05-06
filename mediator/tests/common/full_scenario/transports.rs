use mediator::{Event, ExitTransport, Response};

use super::requests::GenericResponse;

pub struct ATransport {
    pub messages_to_handle: Vec<String>,
}

impl ExitTransport for ATransport {
    fn can_handle(&self, message: &dyn Event) -> bool {
        self.messages_to_handle
            .contains(&message.name().to_string())
    }

    fn request(&self, _message: &dyn Event) -> Option<Box<dyn Response>> {
        return Some(Box::new(GenericResponse {
            data: "Transport A response".to_string(),
        }));
    }
}

pub struct BTransport {
    pub messages_to_handle: Vec<String>,
}

impl ExitTransport for BTransport {
    fn can_handle(&self, message: &dyn Event) -> bool {
        self.messages_to_handle
            .contains(&message.name().to_string())
    }

    fn request(&self, _message: &dyn Event) -> Option<Box<dyn Response>> {
        return Some(Box::new(GenericResponse {
            data: "Transport B response".to_string(),
        }));
    }
}
