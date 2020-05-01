//! Message tracing macros

/// Trace a decoding event
macro_rules! trace {
    ($decoder:expr, $c:expr, $msg:expr, $($arg:tt)*) => {
        let mut prefix: heapless::String<heapless::consts::U128> = heapless::String::new();
        for _ in 0..$decoder.depth() {
            prefix.push($c).unwrap();
        }
        log::trace!(concat!("{}", $msg), &prefix, $($arg)*);
    }
}

/// Trace the beginning of a message component being decoded
macro_rules! begin {
    ($decoder:expr, $msg:expr, $($arg:tt)*) => {
        trace!($decoder, '+', $msg, $($arg)*);
    }
}
