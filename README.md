# rust-mediator

This is a personal playground for learning Rust. 
It is a simple mediator pattern implementation. 
It is not intended for production use, at least not yet.

## Usage

```rust
    let mut mediator = Mediator::new();

    mediator.register_transport(Box::new(ATransport {
        messages_to_handle: vec!["A1Message".to_string(), "A2Message".to_string()],
    }));

    mediator.register_transport(Box::new(BTransport {
        messages_to_handle: vec!["B1Message".to_string()],
    }));

    let a1_message = A1Message {
        name: "something".to_string(),
        age: 10,
    };

    let a2_message = A2Message;
    let b1_message = B1Message;
    let no_transport_message = NoTransportMessage;

    let a1_response = mediator.send(&a1_message);
    assert!(a1_response.is_some());
    assert_eq!(a1_response.unwrap().data(), "Transport A response");

    let a2_response = mediator.send(&a2_message);
    assert!(a2_response.is_some());
    assert_eq!(a2_response.unwrap().data(), "Transport A response");

    let b1_response = mediator.send(&b1_message);
    assert!(b1_response.is_some());
    assert_eq!(b1_response.unwrap().data(), "Transport B response");

    let no_transport_response = mediator.send(&no_transport_message);
    assert!(no_transport_response.is_none());
```	


## Roadmap

- [ ] Add handlers
- [ ] Add handlers registration/subscription
- [ ] Add async support
- [ ] Add send timeout
- [ ] Add publish support
- [ ] Improve message identification (macro?) and use as discriminator for handlers
- [ ] Make messages and responses the more generic possible
- [ ] Add transport support for in memory
- [ ] Add transport support for RabbitMQ
- [ ] Think about routing preferences (e.g. prefer in memory over RabbitMQ, because of performance reasons)



## License

Licensed under MIT license ([LICENSE](LICENSE) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, shall be licensed as above, without any additional terms or conditions.







