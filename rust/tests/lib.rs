extern crate zser;

use zser::decoder::Decoder;
use zser::parser::Parser;
use zser::varint;

mod message_examples;
mod varint_examples;

#[test]
fn message_examples() {
    let examples = message_examples::load();

    for example in examples {
        let mut parser = Parser::new(Decoder::new());

        if example.success {
            let result = parser.parse(&example.encoded);

            if result.is_err() {
                panic!("Error: {}: {}", example.name, result.err().unwrap());
            }

            let value = parser.finish().expect("finished");
            let expected = message_examples::decode_value(&example.decoded.expect("decoded"));
            assert_eq!(value, expected);
        } else if parser.parse(&example.encoded).is_ok() {
            panic!("{}: expected error but example parsed successfully",
                   example.name);
        }
    }
}

#[test]
fn varint_encode() {
    let examples = varint_examples::load();
    let mut output = [0u8; 9];

    for example in examples {
        let len = varint::encode(example.value, &mut output);
        assert_eq!(&output[..len], &example.encoded[..]);
    }
}

#[test]
fn varint_decode() {
    let examples = varint_examples::load();

    for example in examples {
        let mut encoded_ref = &example.encoded[..];
        assert_eq!(varint::decode(&mut encoded_ref).unwrap(), example.value);
        assert_eq!(encoded_ref, b"");
    }
}
