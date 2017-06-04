#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate zser;

use zser::varint;

fuzz_target!(|input: &[u8]| {
    let mut input_ref = &input[..];

    let value = match varint::decode(&mut input_ref) {
        Ok(v) => v,
        Err(_) => return
    };

    let mut output = [0u8; 9];
    let len = varint::encode(value, &mut output);

    if &input[..len] != &output[..len] {
        panic!("input and output do not match: input: {:?}, output: {:?}, len: {:?}", input, output, len);
    }
});
