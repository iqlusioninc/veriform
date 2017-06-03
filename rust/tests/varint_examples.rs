extern crate data_encoding;
extern crate serde_json;
extern crate zser;

use self::data_encoding::HEXLOWER;
pub use self::serde_json::Value as JsonValue;
use std::fs::File;
use std::io::Read;
use std::path::Path;

/// Varint examples (with varint.tjson structure)
// TODO: switch to the tjson crate (based on serde)
#[derive(Debug)]
pub struct VarintExample {
    pub value: u64,
    pub encoded: Vec<u8>,
}

/// Load examples from varint.tjson
pub fn load() -> Vec<VarintExample> {
    load_from_file(Path::new("../vectors/varint.tjson"))
}

/// Load examples from a file at the given path
pub fn load_from_file(path: &Path) -> Vec<VarintExample> {
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
            VarintExample {
                value: ex["value:u"]
                    .as_str()
                    .expect("string data")
                    .parse()
                    .expect("unsigned integer value"),
                encoded: HEXLOWER
                    .decode(ex["encoded:d16"]
                                .as_str()
                                .expect("encoded example")
                                .as_bytes())
                    .expect("hex encoded"),
            }
        })
        .collect()
}
