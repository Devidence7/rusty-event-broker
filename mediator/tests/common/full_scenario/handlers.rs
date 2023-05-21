use std::sync::Arc;

use mediator::{MessageName, RequestHandler};
use mediator_macros::message;

use super::requests::{A1Request, A2Request, B1Request, GenericResponse};

#[message(A1RequestName)]
pub struct A1RequestHandler {}

impl RequestHandler for A1RequestHandler {
    fn handle_request(
        &self,
        request: Arc<dyn MessageName>,
    ) -> Result<Arc<dyn MessageName>, String> {
        let request = request
            .as_any()
            .downcast_ref::<A1Request>()
            .expect("Request is not an A1Request");

        Ok(Arc::new(GenericResponse {
            data: format!("A1RequestHandler: {}", request.message_name()),
        }))
    }
}

#[message(A2RequestName)]
pub struct A2RequestHandler {}

impl RequestHandler for A2RequestHandler {
    fn handle_request(
        &self,
        request: Arc<dyn MessageName>,
    ) -> Result<Arc<dyn MessageName>, String> {
        let request = request
            .as_any()
            .downcast_ref::<A2Request>()
            .expect("Request is not an A2Request");

        Ok(Arc::new(GenericResponse {
            data: format!("A2RequestHandler: {}", request.message_name()),
        }))
    }
}

#[message(B1RequestName)]
pub struct B1RequestHandler {}

impl RequestHandler for B1RequestHandler {
    fn handle_request(
        &self,
        request: Arc<dyn MessageName>,
    ) -> Result<Arc<dyn MessageName>, String> {
        let request = request
            .as_any()
            .downcast_ref::<B1Request>()
            .expect("Request is not an B1Request");

        Ok(Arc::new(GenericResponse {
            data: format!("B1RequestHandler: {}", request.message_name()),
        }))
    }
}
