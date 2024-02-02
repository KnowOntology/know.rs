// This is free and unencumbered software released into the public domain.

use iri_string::types::IriString;

/// See: https://en.wikipedia.org/wiki/Software_release_life_cycle
#[cfg(feature = "serde")]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Default, PartialEq, PartialOrd)]
pub struct SoftwareRelease {
    pub version: String,
    pub date: String,
    pub link: Option<IriString>,
}
