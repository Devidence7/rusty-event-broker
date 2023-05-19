use std::sync::Arc;

use crate::{MessageName, Request, Response};
use async_trait::async_trait;

#[async_trait]
pub trait RequestHandler<TRequest, TResponse>: MessageName
where
    TRequest: Request,
    TResponse: Response,
{
    async fn handle_request(&self, request: Arc<TRequest>) -> Arc<TResponse>;
}
