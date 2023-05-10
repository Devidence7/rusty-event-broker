# RustyEventBroker 
A mediator library that brokers the communication between different message transports.

:construction: This is a personal playground for learning Rust. 
It is a simple mediator pattern implementation. 
It is not intended for production use, at least not yet. :construction:

## Usage
First define your transports and your message handlers. 

```rust
    let mut broker = Broker::new();

    let in_memory_transport = Rc::new(Mutex::new(InMemoryTransport::new()));

    broker.register_exit_transport(in_memory_transport.clone());
    broker.register_entry_transport(in_memory_transport);

    broker.register_request_handler(Rc::new(A1RequestHandler {}));
    broker.register_request_handler(Rc::new(A2RequestHandler {}));
    broker.register_request_handler(Rc::new(B1RequestHandler {}));
```	

Example of a message handler. 
```rust
impl RequestHandler for A1RequestHandler {
    fn handle(&self, _request: Rc<dyn Request>) -> Option<Rc<dyn Response>> {
        let response = GenericResponse {
            data: "A1 Handler Response".to_string(),
        };

        return Some(Rc::new(response));
    }

    fn name(&self) -> &str {
        "A1Message"
    }
}
```

Then send your messages.

```rust
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
```


## Roadmap

- [X] Add handlers
- [X] Add request support
- [ ] Add request async support
- [ ] Add request timeout
- [ ] Add publish support
- [ ] Add send support
- [ ] Improve message identification (macro?) and use as discriminator for handlers
- [ ] Make handlers, messages the more simple and generic as possible. Create a macro to generate the boilerplate code.
- [ ] Add transport support for in memory
- [ ] Add transport support for RabbitMQ
- [ ] Think about routing preferences (e.g. prefer in memory over RabbitMQ, because of performance reasons)



## License

Licensed under MIT license ([LICENSE](LICENSE) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, shall be licensed as above, without any additional terms or conditions.







