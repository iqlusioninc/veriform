//! Support for the `#[digest(...)]` attribute.
//!
//! This attribute allows a member of a struct to contain a digest computed at
//! the time a message is decoded.

use darling::FromField;
use std::str::FromStr;

/// Parsed `#[digest(...)]` attribute.
///
/// At present, the only meaningful usage of this attribute is:
///
/// ```text
/// #[digest(alg = "sha256")]
/// digest: Option<[u8; 32]>
/// ```
///
/// This indicates that the given field of a struct should be populated with
/// the SHA-256 Verihash digest of a message.
///
/// This attribute is presently ignored at encoding time, but recommended
/// (and possibly required in the future) to be set to `None`.
///
/// If you'd like, you can use the [`veriform::Sha256Digest`] type alias
/// instead of `[u8; 32]`.
///
/// [`veriform::Sha256Digest`]: https://docs.rs/veriform/latest/veriform/type.Sha256Digest.html
#[derive(Debug, FromField)]
#[darling(attributes(digest))]
pub(crate) struct Attrs {
    /// Algorithm that identifies this field
    alg: String,
}

impl Attrs {
    /// Parse the algorithm selected in the attribute
    pub fn alg(&self) -> Algorithm {
        self.alg.parse().unwrap_or_else(|_| {
            panic!("error parsing algorithm: {}", self.alg);
        })
    }
}

/// Supported digest algorithms
pub(crate) enum Algorithm {
    /// SHA-256
    Sha256,
}

impl FromStr for Algorithm {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "sha256" => Ok(Algorithm::Sha256),
            _ => Err(()),
        }
    }
}
