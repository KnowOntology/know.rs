// This is free and unencumbered software released into the public domain.

use super::{Person, SoftwareLicense, SoftwarePackage};
use iri_string::types::IriString;
use std::str::FromStr;

#[cfg(feature = "serde")]
use serde_with::serde_as;

/// See: https://en.wikipedia.org/wiki/Software
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval, serde_as)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default, PartialEq, PartialOrd)]
pub struct SoftwareProject {
    pub name: String,

    pub version: String,

    pub summary: String,

    pub description: String,

    #[cfg_attr(feature = "serde", serde(default))]
    pub license: SoftwareLicense,

    #[cfg_attr(
        feature = "serde",
        serde(alias = "author", default),
        serde_as(as = "serde_with::OneOrMany<serde_with::PickFirst<(_, serde_with::DisplayFromStr)>>")
    )]
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
