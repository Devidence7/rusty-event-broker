use std::rc::Rc;

use mediator::{Broker, Request, RequestHandler, Response};

struct TestRequestHandler {}
impl RequestHandler for TestRequestHandler {
    fn handle(&self, _request: Rc<dyn Request>) -> Option<Rc<dyn Response>> {
        None
    }

    fn name(&self) -> &str {
        "Test"
    }
}

#[test]
fn test_request_handler() {
    let mut broker = Broker::new();
    let handler = TestRequestHandler {};
    broker.register_request_handler(&handler);
}
