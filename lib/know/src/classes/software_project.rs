// This is free and unencumbered software released into the public domain.

use crate::prelude::*;
use super::{PersonRef, SoftwareLicense, SoftwarePackageRef, ThingLike};
use std::str::FromStr;

#[cfg(feature = "serde")]
use serde_with::serde_as;

/// See: https://en.wikipedia.org/wiki/Software
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval, serde_as)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default, PartialEq, PartialOrd)]
pub struct SoftwareProject {
    pub name: Name,

    pub version: String,

    pub summary: String,

    pub description: String,

    #[cfg_attr(feature = "serde", serde(default))]
    pub license: SoftwareLicense,

    #[cfg_attr(
        feature = "serde",
        serde(default, alias = "author"),
        serde_as(
            as = "serde_with::OneOrMany<serde_with::PickFirst<(_, serde_with::DisplayFromStr)>>"
        )
    )]
    pub authors: Vec<PersonRef>,

    pub email: Option<Email>,

    pub link: Option<IRI>,

    pub github: Option<IRI>,

    pub package: Option<SoftwarePackageRef>,
}

impl ThingLike for SoftwareProject {
    fn id(&self) -> Option<&str> {
        None
    }

    fn name(&self) -> &Name {
        &self.name
    }
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
