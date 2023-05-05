# rust-mediator

This is a personal playground for learning Rust. 
It is a simple mediator pattern implementation. 
It is not intended for production use, at least not yet.

## Usage

```rust
    let mut mediator = Mediator::new();

    mediator.register_handler(Box::new(AHandler {
        messages_to_handle: vec!["A1Message".to_string(), "A2Message".to_string()],
    }));

    mediator.register_handler(Box::new(BHandler {
        messages_to_handle: vec!["B1Message".to_string()],
    }));

    let a1_message = A1Message {
        name: "something".to_string(),
        age: 10,
    };

    let a2_message = A2Message;
    let b1_message = B1Message;
    let no_handler_message = NoHadlerMessage;

    let a1_response = mediator.send(&a1_message);
    assert!(a1_response.is_some());
    assert_eq!(a1_response.unwrap().data(), "AHandler response");

    let a2_response = mediator.send(&a2_message);
    assert!(a2_response.is_some());
    assert_eq!(a2_response.unwrap().data(), "AHandler response");

    let b1_response = mediator.send(&b1_message);
    assert!(b1_response.is_some());
    assert_eq!(b1_response.unwrap().data(), "BHandler response");

    let no_handler_response = mediator.send(&no_handler_message);
    assert!(no_handler_response.is_none());
```	


## Roadmap

- [ ] Add async support
- [ ] Add send timeout
- [ ] Add publish support
- [ ] Improve message identification (macro?) and use as discriminator for handlers
- [ ] Make messages and responses the more generic possible
- [ ] Create channels
- [ ] Add chanel support for in memory
- [ ] Add chanel support for RabbitMQ
- [ ] Think about routing preferences (e.g. prefer in memory over RabbitMQ, because of performance reasons)



## License

Licensed under MIT license ([LICENSE](LICENSE) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, shall be licensed as above, without any additional terms or conditions.







