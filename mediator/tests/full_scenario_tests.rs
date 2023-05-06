#![feature(lazy_cell)]

use std::rc::Rc;

use crate::common::full_scenario::requests::*;

mod common;

#[test]
fn multiple_transports_test() {
    let mut broker = common::full_scenario::setup();

    let a1_message = A1Message {
        name: "something".to_string(),
        age: 10,
    };

    let a2_message = A2Message;
    let b1_message = B1Message;
    let no_transport_message = NoTransportMessage;

    let a1_response = broker.request(Rc::new(a1_message));
    assert!(a1_response.is_some());
    assert_eq!(a1_response.unwrap().data(), "A1 Handler Response");

    let a2_response = broker.request(Rc::new(a2_message));
    assert!(a2_response.is_some());
    assert_eq!(a2_response.unwrap().data(), "A2 Handler Response");

    let b1_response = broker.request(Rc::new(b1_message));
    assert!(b1_response.is_some());
    assert_eq!(b1_response.unwrap().data(), "B1 Handler Response");

    let no_transport_response = broker.request(Rc::new(no_transport_message));
    assert!(no_transport_response.is_none());
}
