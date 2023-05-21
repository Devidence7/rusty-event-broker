#![feature(lazy_cell)]

use std::sync::Arc;

use common::full_scenario::setup;
use mediator::{Broker, MessageName};

use crate::common::full_scenario::requests::*;

mod common;

#[test]
fn multiple_transports_test() {
    let broker: Broker = common::full_scenario::setup();

    let a1_request = A1Request {
        name: "something".to_string(),
        age: 10,
    };

    let a2_request = A2Request {};
    let b1_request = B1Request {};
    let no_transport_request = NoTransportRequest {};

    let a1_response_result: Result<Arc<dyn MessageName>, String> =
        broker.handle_request(Arc::new(a1_request));

    assert!(a1_response_result.is_ok());
    let a1_response: Arc<dyn MessageName> = a1_response_result.unwrap();
    let generic_response = a1_response
        .as_any()
        .downcast_ref::<GenericResponse>()
        .unwrap();
    assert_eq!(generic_response.data, "A1RequestHandler: A1RequestName");

    let a2_response_result: Result<Arc<dyn MessageName>, String> =
        broker.handle_request(Arc::new(a2_request));
    assert!(a2_response_result.is_ok());
    let a2_response: Arc<dyn MessageName> = a2_response_result.unwrap();
    let generic_response = a2_response
        .as_any()
        .downcast_ref::<GenericResponse>()
        .unwrap();
    assert_eq!(generic_response.data, "A2RequestHandler: A2RequestName");

    let b1_response_result: Result<Arc<dyn MessageName>, String> =
        broker.handle_request(Arc::new(b1_request));
    assert!(b1_response_result.is_ok());
    let b1_response: Arc<dyn MessageName> = b1_response_result.unwrap();
    let generic_response = b1_response
        .as_any()
        .downcast_ref::<GenericResponse>()
        .unwrap();
    assert_eq!(generic_response.data, "B1RequestHandler: B1RequestName");

    let no_transport_response_result: Result<Arc<dyn MessageName>, String> =
        broker.handle_request(Arc::new(no_transport_request));
    assert!(no_transport_response_result.is_err());
    let error = no_transport_response_result.err().unwrap();
    assert_eq!(
        error,
        "No transport found for message: NoTransportRequestName"
    );

    setup();
}
