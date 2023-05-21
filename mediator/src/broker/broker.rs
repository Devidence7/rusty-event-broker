use std::sync::{Arc, Mutex};

use crate::{EntryTransport, ExitTransport, MessageName};

pub struct Broker {
    exit_transports: Vec<Arc<Mutex<dyn ExitTransport>>>,
    entry_transports: Vec<Arc<Mutex<dyn EntryTransport>>>,
}

impl Broker {
    pub fn new() -> Self {
        Self {
            exit_transports: Vec::new(),
            entry_transports: Vec::new(),
        }
    }

    pub fn register_exit_transport(&mut self, transport: Arc<Mutex<dyn ExitTransport>>) {
        self.exit_transports.push(transport);
    }

    pub fn register_entry_transport(&mut self, transport: Arc<Mutex<dyn EntryTransport>>) {
        self.entry_transports.push(transport);
    }

    pub fn handle_request<'a, TRequest, TResponse>(
        &self,
        request: TRequest,
    ) -> Result<Arc<dyn MessageName>, String>
    where
        TRequest: MessageName + 'static,
        TResponse: MessageName + 'static,
    {
        // TODO: Find exit transport that can handle request
        let transport = self.exit_transports[0].clone();

        // Tranform Transport into ExitTransport
        let exit_transport = transport as Arc<Mutex<dyn ExitTransport>>;

        // Create a let binding for the response object
        let response = {
            let request = Arc::new(request);
            exit_transport.lock().unwrap().request(request)?
        };

        return Ok(response);
    }
}
