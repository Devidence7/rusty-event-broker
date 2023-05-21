#![feature(lazy_cell)]

use mediator::Broker;

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

    let a1_response_result = broker.handle_request::<A1Request, GenericResponse>(a1_request);
    assert!(a1_response_result.is_ok());
    let a1_response = a1_response_result.unwrap();
    assert_eq!(a1_response.message_name(), "GenericResponseName");
    assert_eq!(
        a1_response
            .as_any()
            .downcast_ref::<GenericResponse>()
            .unwrap()
            .data,
        "A1RequestHandler: A1RequestName"
    );

    let a2_response_result = broker.handle_request::<A2Request, GenericResponse>(a2_request);
    assert!(a2_response_result.is_ok());
    let a2_response = a2_response_result.unwrap();
    assert_eq!(a2_response.message_name(), "GenericResponseName");
    assert_eq!(
        a2_response
            .as_any()
            .downcast_ref::<GenericResponse>()
            .unwrap()
            .data,
        "A2RequestHandler: A2RequestName"
    );

    let b1_response_result = broker.handle_request::<B1Request, GenericResponse>(b1_request);
    assert!(b1_response_result.is_ok());
    let b1_response = b1_response_result.unwrap();
    assert_eq!(b1_response.message_name(), "GenericResponseName");
    assert_eq!(
        b1_response
            .as_any()
            .downcast_ref::<GenericResponse>()
            .unwrap()
            .data,
        "B1RequestHandler: B1RequestName"
    );

    let no_transport_response_result =
        broker.handle_request::<NoTransportRequest, GenericResponse>(no_transport_request);
    assert!(no_transport_response_result.is_err());
    let no_transport_response = no_transport_response_result.err().unwrap();
    assert_eq!(
        no_transport_response,
        "No transport found for message: NoTransportRequestName"
    );
}
