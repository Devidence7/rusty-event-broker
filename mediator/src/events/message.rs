use std::any::Any;

pub trait MessageName: Send + Sync + 'static {
    fn message_name(&self) -> &'static str;

    fn as_any(&self) -> &dyn Any;
}
