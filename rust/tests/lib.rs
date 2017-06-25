extern crate zser;

use zser::decoder::Decoder;
use zser::parser::Parser;
use zser::varint;

mod test_vectors;

use test_vectors::{MessageExample, VarintExample, decode_value};

#[test]
fn message_examples() {
    let examples = MessageExample::load_all();

    for example in examples {
        let mut parser = Parser::new(Decoder::new());

        if example.success {
            let result = parser.parse(&example.encoded);

            if result.is_err() {
                panic!("Error: {}: {}", example.name, result.err().unwrap());
            }

            let value = parser.finish().expect("finished");
            let expected = decode_value(&example.decoded.expect("decoded"));
            assert_eq!(value, expected);
        } else if parser.parse(&example.encoded).is_ok() {
            panic!(
                "{}: expected error but example parsed successfully",
                example.name
            );
        }
    }
}

#[test]
fn varint_encode() {
    let examples = VarintExample::load_all();
    let mut output = [0u8; 9];

    for example in examples {
        if !example.success {
            continue;
        }

        let len = varint::encode(example.value.expect("integer value"), &mut output);
        assert_eq!(&output[..len], &example.encoded[..]);
    }
}

#[test]
fn varint_decode() {
    let examples = VarintExample::load_all();

    for example in examples {
        let mut encoded_ref = &example.encoded[..];

        if example.success {
            assert_eq!(
                varint::decode(&mut encoded_ref).unwrap(),
                example.value.expect("integer value")
            );
            assert_eq!(encoded_ref, b"");
        } else if varint::decode(&mut encoded_ref).is_ok() {
            panic!(
                "expected error but example parsed successfully: {:?}",
                example.encoded
            );
        }
    }
}
