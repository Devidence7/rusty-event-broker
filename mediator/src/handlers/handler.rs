use crate::{Request, Response};
use async_trait::async_trait;
use std::sync::Arc;

pub trait MessageHandler {
    fn handle_message_name(self) -> &'static str;
}

/// This macro implements the MessageHandler trait for the given type.
/// Should be use as follows:
/// ```
/// impl_message_handler!(MyMessageHandler, "MyMessage");
/// struct MyMessageHandler {}
/// ```
#[macro_export]
macro_rules! impl_message_handler {
    ($handler:ty, $message_name:expr) => {
        impl $crate::MessageHandler for $handler {
            fn handle_message_name(self) -> &'static str {
                $message_name
            }
        }
    };
}

#[async_trait]
pub trait RequestHandler {
    async fn handle(self, request: Box<dyn Request>) -> Result<Box<dyn Response>, ()>;
}
