use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;

use crate::{EntryTransport, ExitTransport, Request, RequestHandler, Response};

pub struct InMemoryTransport {
    handlers: HashMap<String, Arc<dyn RequestHandler<dyn Request, dyn Response>>>,
}

impl InMemoryTransport {
    pub fn new() -> Self {
        InMemoryTransport {
            handlers: HashMap::new(),
        }
    }
}

impl EntryTransport for InMemoryTransport {
    fn register_request_handler(
        &mut self,
        handler: Arc<dyn RequestHandler<dyn Request, dyn Response>>,
    ) {
        self.handlers
            .insert(handler.message_name().to_string(), handler);
    }
}

#[async_trait]
impl ExitTransport for InMemoryTransport {
    async fn request<TRequest, TResponse>(
        &self,
        request: Arc<TRequest>,
    ) -> Result<Arc<TResponse>, String>
    where
        TRequest: Request,
        TResponse: Response,
    {
        let binding = self.handlers.get(request.message_name()).ok_or_else(|| {
            format!(
                "No handler registered for message {}",
                request.message_name()
            )
        })?;

        let handler = binding
            .as_any()
            .downcast_ref::<&dyn RequestHandler<TRequest, TResponse>>()
            .ok_or_else(|| {
                format!(
                    "Handler for message {} is not of the correct type",
                    request.message_name()
                )
            })?;

        let response = handler.handle_request(request).await;

        Ok(response)
    }
}
