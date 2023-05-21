use std::sync::Arc;

use crate::MessageName;

pub trait RequestHandler: MessageName {
    fn handle_request(&self, request: Arc<dyn MessageName>)
        -> Result<Arc<dyn MessageName>, String>;
}
