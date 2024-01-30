// This is free and unencumbered software released into the public domain.

use super::{Person, SoftwareLicense};

/// See: https://en.wikipedia.org/wiki/Software
#[cfg(feature = "serde")]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Default, PartialEq, PartialOrd)]
pub struct SoftwareProject {
    pub name: String,
    pub version: String,
    pub summary: String,
    pub description: String,
    pub license: SoftwareLicense,
    pub authors: Vec<Person>,
    pub email: Option<String>,
    pub link: Option<String>,
    pub github: Option<String>,
}
