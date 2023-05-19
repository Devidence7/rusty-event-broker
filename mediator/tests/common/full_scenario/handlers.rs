use std::sync::Arc;

use async_trait::async_trait;

use mediator::{MessageName, Request, RequestHandler, Response};
use mediator_macros::request_handler;

use super::requests::{A1Message, GenericResponse};

struct A1RequestHandler {}
request_handler!(
    A1RequestHandler,
    A1Message,
    GenericResponse,
    async fn handle_request(
        &self,
        request: Arc<A1Message>,
    ) -> Result<Arc<GenericResponse>, String> {
        Ok(Arc::new(GenericResponse {
            data: format!("Hello, {}!", request.name),
        }))
    }
);
