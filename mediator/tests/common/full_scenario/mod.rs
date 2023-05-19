use std::{rc::Rc, sync::Mutex};

use mediator::{Broker, ExitTransport, InMemoryTransport};

use self::handlers::{A1RequestHandler, A2RequestHandler, B1RequestHandler};

pub mod handlers;
pub mod requests;
pub mod transports;

pub fn setup() -> InMemoryTransport {
    // let mut broker = Broker::new();

    let in_memory_transport = Rc::new(Mutex::new(InMemoryTransport::new()));

    in_memory_transport.register_request_handler(Rc::new(A1RequestHandler {}));
    in_memory_transport.register_request_handler(Rc::new(A2RequestHandler {}));
    in_memory_transport.register_request_handler(Rc::new(B1RequestHandler {}));

    // broker.register_exit_transport(in_memory_transport.clone());
    // broker.register_entry_transport(in_memory_transport);

    return in_memory_transport;
}
