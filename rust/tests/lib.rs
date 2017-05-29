extern crate zser;

use zser::decoder::Decoder;
use zser::parser::Parser;

mod message_examples;

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
