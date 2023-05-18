use async_trait::async_trait;

use mediator::{impl_message_handler, Request, RequestHandler, Response};

use super::requests::GenericResponse;

pub struct A1RequestHandler {}

impl_message_handler!(A1RequestHandler, "A1Message");

#[async_trait]
impl RequestHandler for A1RequestHandler {
    async fn handle(self, request: Box<dyn Request>) -> Result<Box<dyn Response>, ()> {
        let response = GenericResponse {
            data: "A1 Handler Response".to_string(),
        };

        return Ok(Box::new(response));
    }
}

pub struct A2RequestHandler {}

impl_message_handler!(A2RequestHandler, "A2Message");

#[async_trait]
impl RequestHandler for A2RequestHandler {
    async fn handle(self, request: Box<dyn Request>) -> Result<Box<dyn Response>, ()> {
        let response = GenericResponse {
            data: "A2 Handler Response".to_string(),
        };

        return Ok(Box::new(response));
    }
}

pub struct B1RequestHandler {}

impl_message_handler!(B1RequestHandler, "B1Message");

#[async_trait]
impl RequestHandler for B1RequestHandler {
    async fn handle(self, request: Box<dyn Request>) -> Result<Box<dyn Response>, ()> {
        let response = GenericResponse {
            data: "B1 Handler Response".to_string(),
        };

        return Ok(Box::new(response));
    }
}
