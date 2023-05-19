use std::{any::Any, sync::Arc};

pub trait MessageName: Send + Sync + 'static {
    fn message_name(&self) -> &'static str;

    fn as_any(&self) -> &dyn Any;
}
