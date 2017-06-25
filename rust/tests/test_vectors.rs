extern crate data_encoding;
extern crate serde_json;
extern crate zser;

use self::data_encoding::HEXLOWER;
pub use self::serde_json::Value as JsonValue;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use zser::value::Map as ZserMap;
use zser::value::Value as ZserValue;

/// Message examples (with messages.tjson structure)
// TODO: switch to the tjson crate (based on serde)
#[derive(Debug)]
pub struct MessageExample {
    pub name: String,
    pub description: String,
    pub success: bool,
    pub encoded: Vec<u8>,
    pub decoded: Option<JsonValue>,
}

impl MessageExample {
    /// Load examples from messages.tjson
    pub fn load_all() -> Vec<Self> {
        Self::load_from_file(Path::new("../vectors/messages.tjson"))
    }

    /// Load examples from a file at the given path
    pub fn load_from_file(path: &Path) -> Vec<Self> {
        let mut file = File::open(&path).expect("valid messages.tjson");
        let mut tjson_string = String::new();
        file.read_to_string(&mut tjson_string)
            .expect("messages.tjson read successfully");

        let tjson: serde_json::Value =
            serde_json::from_str(&tjson_string).expect("messages.tjson parses successfully");
        let examples = &tjson["examples:A<O>"]
            .as_array()
            .expect("messages.tjson examples array");

        examples
            .into_iter()
            .map(|ex| {
                Self {
                    name: ex["name:s"].as_str().expect("example name").to_owned(),
                    description: ex["description:s"]
                        .as_str()
                        .expect("example description")
                        .to_owned(),
                    success: ex["success:b"].as_bool().expect("boolean success value"),
                    encoded: HEXLOWER
                        .decode(ex["encoded:d16"]
                            .as_str()
                            .expect("encoded example")
                            .as_bytes())
                        .expect("hex encoded"),
                    decoded: ex.get("decoded:O").cloned(),
                }
            })
            .collect()
    }
}

/// Varint examples (with varint.tjson structure)
// TODO: switch to the tjson crate (based on serde)
#[derive(Debug)]
pub struct VarintExample {
    pub value: Option<u64>,
    pub encoded: Vec<u8>,
    pub success: bool,
}

impl VarintExample {
    /// Load examples from varint.tjson
    pub fn load_all() -> Vec<Self> {
        Self::load_from_file(Path::new("../vectors/varint.tjson"))
    }

    /// Load examples from a file at the given path
    pub fn load_from_file(path: &Path) -> Vec<Self> {
        let mut file = File::open(&path).expect("valid varint.tjson");
        let mut tjson_string = String::new();
        file.read_to_string(&mut tjson_string)
            .expect("varint.tjson read successfully");

        let tjson: serde_json::Value =
            serde_json::from_str(&tjson_string).expect("varint.tjson parses successfully");
        let examples = &tjson["examples:A<O>"]
            .as_array()
            .expect("varint.tjson examples array");

        examples
            .into_iter()
            .map(|ex| {
                let success = ex["success:b"].as_bool().expect("success boolean");

                let value = if success {
                    Some(ex["value:u"]
                        .as_str()
                        .expect("value string")
                        .parse()
                        .expect("unsigned integer value"))
                } else {
                    None
                };

                Self {
                    value: value,
                    encoded: HEXLOWER
                        .decode(ex["encoded:d16"]
                            .as_str()
                            .expect("encoded example")
                            .as_bytes())
                        .expect("hex encoded"),
                    success: success,
                }
            })
            .collect()
    }
}

/// Convert a decoded value from (T)JSON to the corresponding `ZserValue`
pub fn decode_value(value: &JsonValue) -> ZserValue {
    if let JsonValue::Object(ref input_map) = *value {
        let mut output_map = ZserMap::new();

        for (key, encoded_value) in input_map {
            let mut parts = key.split(':');

            let id: u64 = parts
                .next()
                .expect("colon delimited tag")
                .parse()
                .expect("numeric id");

            let tag = parts.next().expect("colon delimited tag");

            let decoded_value = match tag {
                "O" => decode_value(encoded_value),
                "d16" => {
                    ZserValue::Data(HEXLOWER
                                        .decode(encoded_value
                                                    .as_str()
                                                    .expect("string data")
                                                    .as_bytes())
                                        .expect("hex encoded"))
                }
                "u" => {
                    ZserValue::UInt(encoded_value
                                        .as_str()
                                        .expect("string data")
                                        .parse()
                                        .expect("unsigned integer value"))
                }
                _ => panic!("unknown tag: '{}'", tag),
            };

            output_map.insert(id, decoded_value);
        }

        ZserValue::Message(output_map)
    } else {
        panic!("can't convert value: {:?}", value);
    }
}
