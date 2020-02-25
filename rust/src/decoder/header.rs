//! Decoder for field headers

use super::{vint64, Event, State, Tag};
use crate::{error::Error, field::Header};
use core::convert::TryFrom;

/// Decoder for field headers
#[derive(Default, Debug)]
pub(super) struct Decoder(vint64::Decoder);

impl Decoder {
    /// Process the given input data, advancing the slice for the amount of
    /// data processed, and returning the new state.
    pub fn decode<'a>(
        mut self,
        input: &mut &'a [u8],
        last_tag: Option<Tag>,
    ) -> Result<(State, Option<Event<'a>>), Error> {
        if let Some(value) = self.0.decode(input)? {
            let header = Header::try_from(value)?;

            // Ensure field ordering is monotonically increasing
            if let Some(tag) = last_tag {
                if header.tag <= tag {
                    return Err(Error::Order { tag: header.tag });
                }
            }

            let event = Event::FieldHeader(header);
            let new_state = State::transition(&event);
            Ok((new_state, Some(event)))
        } else {
            Ok((State::Header(self), None))
        }
    }
}
