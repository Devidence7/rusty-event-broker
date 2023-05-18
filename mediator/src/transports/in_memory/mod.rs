use async_trait::async_trait;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;
use std::sync::Mutex;
use std::{collections::VecDeque, sync::Arc};

use crate::{EntryTransport, ExitTransport, Request, RequestHandler, Response};

pub struct InMemoryQueueTransport {
    request_handlers: HashMap<String, Arc<dyn RequestHandler>>,
    request_queue: VecDeque<Box<dyn Request>>,
}

#[async_trait]
impl ExitTransport for InMemoryQueueTransport {
    fn can_handle_request(&self, message: &dyn Request) -> bool {
        todo!();
    }

    async fn handle(self, request: Box<dyn Request>) -> Result<Box<dyn Response>, ()> {}
}

impl EntryTransport for InMemoryQueueTransport {
    fn listen(&mut self) {}

    fn register_request_handler(&mut self, handler: Arc<dyn RequestHandler>) {
        self.request_handlers
            .insert(handler.handle_message_name().to_string(), handler);
    }
}
