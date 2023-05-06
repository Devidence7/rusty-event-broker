use super::requests::GenericResponse;
use mediator::{ExitTransport, Request, Response};
use std::rc::Rc;

pub struct ATransport {
    pub allowed_requests: Vec<String>,

    // Store in memory
    pub current_requests: Vec<Rc<dyn Request>>,
}

impl ATransport {
    pub fn new() -> Self {
        ATransport {
            allowed_requests: Vec::new(),
            current_requests: Vec::new(),
        }
    }
}

impl ExitTransport for ATransport {
    fn can_handle_request(&self, message: Rc<dyn Request>) -> bool {
        self.allowed_requests.contains(&message.name().to_string())
    }

    fn request(&mut self, message: Rc<dyn Request>) -> Option<Rc<dyn Response>> {
        self.current_requests.push(message);

        let response = GenericResponse {
            data: "A1Response".to_string(),
        };

        return Some(Rc::new(response));
    }
}
