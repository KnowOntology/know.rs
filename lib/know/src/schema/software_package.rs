// This is free and unencumbered software released into the public domain.

use iri_string::types::IriString;
use std::str::FromStr;

/// See: https://en.wikipedia.org/wiki/Software_package
#[cfg(feature = "serde")]
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, PartialEq, PartialOrd)]
pub struct SoftwarePackage {
    pub name: String,
    pub version: String,
    pub link: Option<IriString>,
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
