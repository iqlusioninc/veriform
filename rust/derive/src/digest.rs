//! Support for the `#[digest(...)]` attribute.
//!
//! This attribute allows a member of a struct to contain a digest computed at
//! the time a message is decoded.

use darling::FromField;
use std::str::FromStr;

/// Parsed `#[digest(...)]` attribute
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
