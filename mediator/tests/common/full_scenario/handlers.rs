use std::rc::Rc;

use mediator::{Request, RequestHandler, Response};

use super::requests::GenericResponse;

pub struct A1RequestHandler {}

impl RequestHandler for A1RequestHandler {
    fn handle(&self, _request: Rc<dyn Request>) -> Option<Rc<dyn Response>> {
        let response = GenericResponse {
            data: "A1 Handler Response".to_string(),
        };

        return Some(Rc::new(response));
    }

    fn name(&self) -> &str {
        "A1Message"
    }
}

pub struct A2RequestHandler {}

impl RequestHandler for A2RequestHandler {
    fn handle(&self, _request: Rc<dyn Request>) -> Option<Rc<dyn Response>> {
        let response = GenericResponse {
            data: "A2 Handler Response".to_string(),
        };

        return Some(Rc::new(response));
    }

    fn name(&self) -> &str {
        "A2Message"
    }
}

pub struct B1RequestHandler {}

impl RequestHandler for B1RequestHandler {
    fn handle(&self, _request: Rc<dyn Request>) -> Option<Rc<dyn Response>> {
        let response = GenericResponse {
            data: "B1 Handler Response".to_string(),
        };

        return Some(Rc::new(response));
    }

    fn name(&self) -> &str {
        "B1Message"
    }
}
