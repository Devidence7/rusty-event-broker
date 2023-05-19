#![feature(lazy_cell)]

use crate::common::full_scenario::requests::*;

mod common;

#[test]
fn multiple_transports_test() {
    let mut broker = common::full_scenario::setup();

    let a1_message = A1Message {
        name: "something".to_string(),
        age: 10,
    };

    let a2_message = A2Message {};
    let b1_message = B1Message {};
    let no_transport_message = NoTransportMessage {};

    let a1_response = broker.request(a1_message);
}
