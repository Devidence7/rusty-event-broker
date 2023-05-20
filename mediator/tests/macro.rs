// tests/attribute_macro.rs

use mediator::*;
use mediator_macros::*;

#[message(TestMessage)]
struct TestMessage {
    pub a: i32,
    pub b: i32,
}

#[test]
fn test_message() {
    let msg = TestMessage { a: 1, b: 2 };
    assert_eq!(msg.message_name(), "TestMessage");
    assert_eq!(msg.as_any().downcast_ref::<TestMessage>().unwrap().a, 1);
    assert_eq!(msg.as_any().downcast_ref::<TestMessage>().unwrap().b, 2);
}

#[test]
fn test_message_any_method() {
    let msg = TestMessage { a: 1, b: 2 };
    assert_eq!(msg.as_any().downcast_ref::<TestMessage>().unwrap().a, 1);
    assert_eq!(msg.as_any().downcast_ref::<TestMessage>().unwrap().b, 2);
}
