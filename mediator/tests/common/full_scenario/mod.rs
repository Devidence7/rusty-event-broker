use mediator::Broker;

use self::transports::{ATransport, BTransport};

pub mod requests;
pub mod transports;

pub fn setup() -> Broker {
    let mut mediator = Broker::new();

    mediator.register_exit_transport(Box::new(ATransport {
        messages_to_handle: vec!["A1Message".to_string(), "A2Message".to_string()],
    }));

    mediator.register_exit_transport(Box::new(BTransport {
        messages_to_handle: vec!["B1Message".to_string()],
    }));

    return mediator;
}
