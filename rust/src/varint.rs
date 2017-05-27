//! Variable-width 64-bit little endian integers

use byteorder::{ByteOrder, LittleEndian};
use errors::*;

/// Encode a 64-bit unsigned integer in zsuint64 form
pub fn encode(value: u64, out: &mut [u8]) -> usize {
    let mut length = 1;
    let mut result = (value << 1) | 1;
    let mut max = 1 << 7;

    while value >= max {
        // 9-byte special case
        if max == 1 << 63 {
            out[0] = 0;
            LittleEndian::write_u64(&mut out[1..9], value);
            return 9;
        }

        result <<= 1;
        max <<= 7;
        length += 1;
    }

    LittleEndian::write_uint(out, result, length);
    length
}

/// Decode a zsuint64-encoded unsigned 64-bit integer
pub fn decode(input: &mut &[u8]) -> Result<u64> {
    let bytes = *input;

    let prefix =
        *bytes
             .first()
             .ok_or_else(|| ErrorKind::TruncatedMessage("missing varint prefix".to_owned()))?;

    if prefix == 0 {
        if bytes.len() >= 9 {
            let result = LittleEndian::read_u64(&bytes[1..9]);
            *input = &bytes[9..];
            return Ok(result);
        } else {
            return Err(ErrorKind::TruncatedMessage("truncated varint".to_owned()).into());
        }
    }

    let count = prefix.trailing_zeros() as usize + 1;

    if bytes.len() < count {
        return Err(ErrorKind::TruncatedMessage("truncated varint".to_owned()).into());
    }

    let result = LittleEndian::read_uint(bytes, count) >> count;
    *input = &bytes[count..];
    Ok(result)
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "bench")]
    use leb128;
    #[cfg(feature = "bench")]
    use test::Bencher;
    use varint;

    #[test]
    fn encode() {
        let mut output = [0u8; 9];

        // 0
        assert_eq!(varint::encode(0, &mut output), 1);
        assert_eq!(output, *b"\x01\0\0\0\0\0\0\0\0");

        // 42
        assert_eq!(varint::encode(42, &mut output), 1);
        assert_eq!(output, *b"U\0\0\0\0\0\0\0\0");

        // 127
        assert_eq!(varint::encode(127, &mut output), 1);
        assert_eq!(output, *b"\xFF\0\0\0\0\0\0\0\0");

        // 128
        assert_eq!(varint::encode(128, &mut output), 2);
        assert_eq!(output, *b"\x02\x02\0\0\0\0\0\0\0");

        // 2**42 - 1
        assert_eq!(varint::encode(4398046511103, &mut output), 6);
        assert_eq!(output, *b"\xE0\xFF\xFF\xFF\xFF\xFF\0\0\0");

        // 2**42
        assert_eq!(varint::encode(4398046511104, &mut output), 7);
        assert_eq!(output, *b"@\x00\x00\x00\x00\x00\x02\0\0");

        // 2**64-2
        assert_eq!(varint::encode(18446744073709551614, &mut output), 9);
        assert_eq!(output, *b"\x00\xFE\xFF\xFF\xFF\xFF\xFF\xFF\xFF");

        // 2**64-1
        assert_eq!(varint::encode(18446744073709551615, &mut output), 9);
        assert_eq!(output, *b"\x00\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF");
    }

    #[test]
    fn decode() {
        let mut remaining;

        // 0 with nothing trailing
        let zero_nothing = b"\x01";
        remaining = &zero_nothing[..];
        assert_eq!(varint::decode(&mut remaining).unwrap(), 0);
        assert_eq!(remaining, b"");

        // 0 with trailing 0
        let zero_trailing_zero = b"\x01\0";
        remaining = &zero_trailing_zero[..];
        assert_eq!(varint::decode(&mut remaining).unwrap(), 0);
        assert_eq!(remaining, b"\0");

        // 42 with trailing 0
        let forty_two_trailing_zero = b"U\0";
        remaining = &forty_two_trailing_zero[..];
        assert_eq!(varint::decode(&mut remaining).unwrap(), 42);
        assert_eq!(remaining, b"\0");

        // 127 with trailing 0
        let one_twenty_seven_trailing_zero = b"\xFF\0";
        remaining = &one_twenty_seven_trailing_zero[..];
        assert_eq!(varint::decode(&mut remaining).unwrap(), 127);
        assert_eq!(remaining, b"\0");

        // 128 with trailing 0
        let one_twenty_eight_trailing_zero = b"\x02\x02\0";
        remaining = &one_twenty_eight_trailing_zero[..];
        assert_eq!(varint::decode(&mut remaining).unwrap(), 128);
        assert_eq!(remaining, b"\0");

        // 2**64-2 with trailing 0
        let maxint_minus_one_trailing_zero = b"\x00\xFE\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0";
        remaining = &maxint_minus_one_trailing_zero[..];
        assert_eq!(varint::decode(&mut remaining).unwrap(),
                   18446744073709551614);
        assert_eq!(remaining, b"\0");

        // 2**64-1 with trailing 0
        let maxint_trailing_zero = b"\x00\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0";
        remaining = &maxint_trailing_zero[..];
        assert_eq!(varint::decode(&mut remaining).unwrap(),
                   18446744073709551615);
        assert_eq!(remaining, b"\0");
    }

    #[cfg(feature = "bench")]
    #[bench]
    fn bench_encode(b: &mut Bencher) {
        let mut output = [0u8; 9];

        // 2**48 + 31337
        b.iter(|| varint::encode(281474976741993, &mut output));
    }

    #[cfg(feature = "bench")]
    #[bench]
    fn bench_decode(b: &mut Bencher) {
        let input = b"\xC04=\x00\x00\x00\x80";

        // 2**48 + 31337
        b.iter(|| {
                   let mut readable = &input[..];
                   varint::decode(&mut readable).unwrap()
               });
    }

    #[cfg(feature = "bench")]
    #[bench]
    fn bench_leb128_encode(b: &mut Bencher) {
        let mut output = [0u8; 9];

        // 2**48 + 31337
        b.iter(|| {
                   let mut writable = &mut output[..];
                   leb128::write::unsigned(&mut writable, 281474976741993).unwrap()
               });
    }

    #[cfg(feature = "bench")]
    #[bench]
    fn bench_leb128_decode(b: &mut Bencher) {
        let input = b"\xE9\xF4\x81\x80\x80\x80@";

        // 2**48 + 31337
        b.iter(|| {
                   let mut readable = &input[..];
                   leb128::read::unsigned(&mut readable).unwrap()
               });
    }
}
