//! Helper functions called from custom derive output

use crate::{
    error::{self, Error},
    field::Tag,
};

/// Make sure input has been consumed
pub fn check_input_consumed(input: &[u8]) -> Result<(), Error> {
    if input.is_empty() {
        Ok(())
    } else {
        Err(error::Kind::TrailingData.into())
    }
}

/// Unknown tag in enum
pub fn unknown_tag(tag: Tag) -> Error {
    error::Kind::FieldHeader {
        tag: Some(tag),
        wire_type: None,
    }
    .into()
}
