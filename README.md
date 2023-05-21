# RustyEventBroker

A mediator library that brokers the communication between different message transports.

:construction: This is a personal playground for learning Rust.
It is a simple mediator pattern implementation.
It is not intended for production use, at least not yet. :construction:

## Usage

Declare your messages.

```rust
#[message(A1RequestName)]
struct A1Request {
    name: String,
    age: u8,
}

#[message(A1Response)]
struct A1Response {
    data: String,
}
```

Declare your message handler.

```rust
#[request_handler(A1RequestHandler, A1RequestName)]
fn handle_request(&self, request: Arc<A1Request>) -> Result<Arc<A1Response>, String> {
    Ok(Arc::new(A1Response {
        data: "A1 Handler Response".to_string(),
    }))
}
```

Add your transports and your message handlers.

```rust
let mut broker = Broker::new();

let in_memory_transport = Arc::new(InMemoryTransport::new());

in_memory_transport.register_request_handler(A1RequestHandler {});
in_memory_transport.register_request_handler(A2RequestHandler {});
in_memory_transport.register_request_handler(B1RequestHandler {});

broker.register_exit_transport(in_memory_transport.clone());
broker.register_entry_transport(in_memory_transport);
```

Then send your messages.

```rust
let a1_message = A1Message {
    name: "something".to_string(),
    age: 10,
};

let a1_response = broker.request::<A1Request, A1Response>(a1_message).unwrap();
assert_eq!(a1_response.data, "A1 Handler Response");
```

## Roadmap

- [x] Add handlers
- [x] Add request support
- [ ] Add request async support. Requires [rust trait async suport](https://blog.rust-lang.org/inside-rust/2023/05/03/stabilizing-async-fn-in-trait.html)
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
