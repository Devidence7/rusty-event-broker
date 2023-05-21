use std::sync::{Arc, Mutex};

use mediator::{Broker, EntryTransport, InMemoryTransport};

use self::handlers::*;

pub mod handlers;
pub mod requests;
pub mod transports;

pub fn setup() -> Broker {
    let mut broker = Broker::new();

    let in_memory_transport = Arc::new(Mutex::new(InMemoryTransport::new()));

    in_memory_transport
        .lock()
        .unwrap()
        .register_request_handler(Arc::new(A1RequestHandler {}));
    in_memory_transport
        .lock()
        .unwrap()
        .register_request_handler(Arc::new(A2RequestHandler {}));
    in_memory_transport
        .lock()
        .unwrap()
        .register_request_handler(Arc::new(B1RequestHandler {}));

    broker.register_exit_transport(in_memory_transport.clone());
    broker.register_entry_transport(in_memory_transport);

    return broker;
}
