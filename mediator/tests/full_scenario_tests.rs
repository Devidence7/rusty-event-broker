use crate::common::full_scenario::requests::*;

mod common;

#[test]
fn multiple_transports_test() {
    let mediator = common::full_scenario::setup();

    let a1_message = A1Message {
        name: "something".to_string(),
        age: 10,
    };

    let a2_message = A2Message;
    let b1_message = B1Message;
    let no_transport_message = NoTransportMessage;

    let a1_response = mediator.send(&a1_message);
    assert!(a1_response.is_some());
    assert_eq!(a1_response.unwrap().data(), "Transport A response");

    let a2_response = mediator.send(&a2_message);
    assert!(a2_response.is_some());
    assert_eq!(a2_response.unwrap().data(), "Transport A response");

    let b1_response = mediator.send(&b1_message);
    assert!(b1_response.is_some());
    assert_eq!(b1_response.unwrap().data(), "Transport B response");

    let no_transport_response = mediator.send(&no_transport_message);
    assert!(no_transport_response.is_none());
}
