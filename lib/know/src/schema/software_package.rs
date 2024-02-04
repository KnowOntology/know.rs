// This is free and unencumbered software released into the public domain.

use super::prelude::*;
use std::str::FromStr;

/// See: https://en.wikipedia.org/wiki/Software_package
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default, PartialEq, PartialOrd)]
pub struct SoftwarePackage {
    pub name: Name,

    pub version: String,

    pub link: Option<IRI>,
}

impl FromStr for SoftwarePackage {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(SoftwarePackage {
            name: input.to_string(),
            ..Default::default()
        })
    }
}
