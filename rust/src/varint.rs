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

#[cfg(feature = "bench")]
mod bench {
    #[cfg(feature = "bench")]
    use leb128;
    #[cfg(feature = "bench")]
    use test::Bencher;
    use varint;

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
