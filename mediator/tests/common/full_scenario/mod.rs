use mediator::Broker;

use self::{handlers::A1RequestHandler, transports::ATransport};

pub mod handlers;
pub mod requests;
pub mod transports;

pub fn setup() -> Broker {
    let mut broker = Broker::new();

    broker.register_exit_transport(Box::new(ATransport::new()));

    broker.register_request_handler(&A1RequestHandler {});

    return broker;
}
