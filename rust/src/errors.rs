//! errors.rs: error types based on error-chain

#![allow(missing_docs)]

error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    errors {
        OversizedMessage(len: usize, limit: usize) {
            description("message too long")
            display("{}-byte message exceeds limit of {} bytes", len, limit)
        }

        MaxDepthExceeded(max: usize) {
            description("max nested message depth exceeded")
            display("max nested message depth of {} bytes exceeded", max)
        }

        TruncatedMessage(msg: String) {
            description("message truncated")
            display("message truncated: {}", msg)
        }

        UnknownWiretype(wiretype: u64) {
            description("unknown wiretype")
            display("unknown wiretype: {}", wiretype)
        }

        UnconsumedMessages(count: usize) {
            description("unconsumed messages remaining in buffer")
            display("unconsumed messages in buffer: {} messages", count)
        }
    }
}
