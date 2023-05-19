use async_trait::async_trait;
use std::sync::Arc;

use crate::{Request, RequestHandler, Response};

#[async_trait]
pub trait ExitTransport: Send + Sync + 'static {
    async fn request<TRequest, TResponse>(
        &self,
        request: Arc<TRequest>,
    ) -> Result<Arc<TResponse>, String>
    where
        TRequest: Request,
        TResponse: Response;
}

pub trait EntryTransport: Send + Sync + 'static {
    fn register_request_handler(
        &mut self,
        handler: Arc<dyn RequestHandler<dyn Request, dyn Response>>,
    );
}
