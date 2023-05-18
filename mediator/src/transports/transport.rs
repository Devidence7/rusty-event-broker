use async_trait::async_trait;
use std::sync::Arc;

use crate::{Request, RequestHandler, Response};

#[async_trait]
pub trait ExitTransport {
    fn can_handle_request(&self, message: &dyn Request) -> bool;

    async fn handle(self, request: Box<dyn Request>) -> Result<Box<dyn Response>, ()>;
}

pub trait EntryTransport {
    fn listen(&mut self);

    fn register_request_handler(&mut self, handler: Arc<dyn RequestHandler>);
}
