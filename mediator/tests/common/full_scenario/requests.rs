use mediator::{MessageName, Request, Response};
use mediator_macros::message;

#[message(A1Request)]
pub struct A1Message {
    pub name: String,
    pub age: u8,
}

impl Request for A1Message {}

#[message(A2Request)]
pub struct A2Message {}

impl Request for A2Message {}

#[message(B1Request)]
pub struct B1Message {}

impl Request for B1Message {}

#[message(NoTransportRequest)]

pub struct NoTransportMessage {}

impl Request for NoTransportMessage {}

#[message(GenericResponse)]
pub struct GenericResponse {
    pub data: String,
}

impl Response for GenericResponse {}
