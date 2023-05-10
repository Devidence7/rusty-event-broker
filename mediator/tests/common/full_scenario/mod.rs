use std::{rc::Rc, sync::Mutex};

use mediator::{Broker, InMemoryTransport};

use self::handlers::{A1RequestHandler, A2RequestHandler, B1RequestHandler};

pub mod handlers;
pub mod requests;
pub mod transports;

pub fn setup() -> Broker {
    let mut broker = Broker::new();

    let in_memory_transport = Rc::new(Mutex::new(InMemoryTransport::new()));

    broker.register_exit_transport(in_memory_transport.clone());
    broker.register_entry_transport(in_memory_transport);

    broker.register_request_handler(Rc::new(A1RequestHandler {}));
    broker.register_request_handler(Rc::new(A2RequestHandler {}));
    broker.register_request_handler(Rc::new(B1RequestHandler {}));

    return broker;
}