// This is free and unencumbered software released into the public domain.

use super::{Person, SoftwareLicense, SoftwarePackage};
use iri_string::types::IriString;
use std::str::FromStr;

/// See: https://en.wikipedia.org/wiki/Software
#[cfg(feature = "serde")]
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, PartialEq, PartialOrd)]
pub struct SoftwareProject {
    pub name: String,
    pub version: String,
    pub summary: String,
    pub description: String,
    #[serde(default)]
    pub license: SoftwareLicense,
    #[serde(default)]
    pub authors: Vec<Person>,
    pub email: Option<String>,
    pub link: Option<IriString>,
    pub github: Option<IriString>,
    pub package: Option<SoftwarePackage>,
}

impl FromStr for SoftwareProject {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(SoftwareProject {
            name: input.to_string(),
            ..Default::default()
        })
    }
}
