use mediator::{MessageName, Request, Response};
use mediator_macros::message_name;

#[message_name("A1Request")]
pub struct A1Message {}
pub struct A1Message {
    pub name: String,
    pub age: u8,
}

impl Request for A1Message {}

#[message_name("A2Request")]
pub struct A2Message {}
pub struct A2Message {}

impl Request for A2Message {}

#[message_name("B1Request")]
pub struct B1Message {}
pub struct B1Message {}

impl Request for B1Message {}

#[message_name("NoTransportRequest")]

pub struct NoTransportMessage {}
pub struct NoTransportMessage {}

impl Request for NoTransportMessage {}

#[message_name("GenericResponse")]
pub struct GenericResponse {}
pub struct GenericResponse {
    pub data: String,
}

impl Response for GenericResponse {}
