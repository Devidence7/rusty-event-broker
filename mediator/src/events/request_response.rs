use crate::MessageName;

pub trait Request: MessageName {}

pub trait Response: MessageName {}
