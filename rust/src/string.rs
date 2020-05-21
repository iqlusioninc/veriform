//! String support

// TODO(tarcieri): use `unicode-normalization` when it fully supports `no_std`. See:
// <https://github.com/unicode-rs/unicode-normalization/issues/58>

use crate::error::{self, Error};

/// Check if a string is canonical.
///
/// We presently limit strings to the ASCII range, but in the future this
/// should be relaxed to allow any UTF-8 string containing normalized Unicode
/// (NOTE: exactly what constitutes canonical/normalized Unicode, e.g. NFC vs
/// NFD, is TBD)
pub fn ensure_canonical(s: &str) -> Result<&str, Error> {
    // TODO(tarcieri): replace this with e.g. `unicode_normalization::is_nfc_quick()`
    if s.is_ascii() {
        Ok(s)
    } else {
        Err(error::Kind::UnicodeNormalization.into())
    }
}
