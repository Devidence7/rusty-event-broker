use mediator::MessageName;
use mediator_macros::message;

#[message(A1RequestName)]
pub struct A1Request {
    pub name: String,
    pub age: u8,
}

#[message(A2RequestName)]
pub struct A2Request {}

#[message(B1RequestName)]
pub struct B1Request {}

#[message(NoTransportRequestName)]
pub struct NoTransportRequest {}

#[message(GenericResponseName)]
pub struct GenericResponse {
    pub data: String,
}
