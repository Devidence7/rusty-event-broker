use std::{rc::Rc, sync::Mutex};

use crate::{EntryTransport, Event, ExitTransport, Request, Response};

pub struct Broker {
    exit_transports: Vec<Rc<Mutex<dyn ExitTransport>>>,
    entry_transports: Vec<Rc<Mutex<dyn EntryTransport>>>,
}

impl Broker {
    pub fn new() -> Self {
        Broker {
            exit_transports: Vec::new(),
            entry_transports: Vec::new(),
        }
    }

    pub fn register_exit_transport(&mut self, transport: Rc<Mutex<dyn ExitTransport>>) {
        self.exit_transports.push(transport);
    }

    pub fn register_entry_transport(&mut self, transport: Rc<Mutex<dyn EntryTransport>>) {
        self.entry_transports.push(transport);
    }

    pub async fn request(&mut self, message: impl Request) -> Result<Box<dyn Response>, ()> {
        todo!();
    }

    pub async fn send(&mut self, message: impl Event) -> Result<(), ()> {
        todo!();
    }

    pub async fn publish(&mut self, message: impl Event) -> Result<(), ()> {
        todo!();
    }
}
