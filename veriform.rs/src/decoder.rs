//! Decodes a Veriform message into `Veriform::Value`

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

use parser::Handler;
use value::{Map, Value};

/// Decode messages to `Veriform::Value`
// TODO: switch to serde
#[derive(Debug, Default)]
pub struct Decoder {
    messages: Vec<Map>,
}

impl Decoder {
    /// Create a new Decoder instance
    pub fn new() -> Self {
        Self { messages: vec![Map::new()] }
    }

    fn current_message(&mut self) -> &mut Map {
        self.messages.last_mut().expect("non-empty messages stack")
    }
}

impl Handler for Decoder {
    type T = Value;

    /// Add a uint64 to the current object
    fn uint64(&mut self, id: u64, value: u64) {
        self.current_message().insert(id, Value::UInt(value));
    }

    /// Add binary data to the current object
    fn binary(&mut self, id: u64, value: &[u8]) {
        self.current_message().insert(
            id,
            Value::Data(Vec::from(value)),
        );
    }

    /// Push down the internal stack, constructing a new object
    fn begin_nested(&mut self) {
        self.messages.push(Map::new());
    }

    /// End a nested object, setting it to the given ID on its parent
    fn end_nested(&mut self, id: u64) {
        let nested_message = self.messages.pop().expect("non-empty messages stack");
        self.current_message().insert(
            id,
            Value::Message(nested_message),
        );
    }

    /// Finish decoding, returning the finished parent object
    fn finish(mut self) -> Value {
        let result = self.messages.pop().expect("non-empty messages stack");

        if !self.messages.is_empty() {
            panic!("messages remaining in stack");
        }

        Value::Message(result)
    }
}
