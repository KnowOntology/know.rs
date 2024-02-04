// This is free and unencumbered software released into the public domain.

use super::{prelude::*, ThingLike};
use std::str::FromStr;

/// See: https://en.wikipedia.org/wiki/Software_release_life_cycle
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default, PartialEq, PartialOrd)]
pub struct SoftwareRelease {
    pub version: String,

    pub date: Option<Date>,

    pub link: Option<IRI>,
}

impl ThingLike for SoftwareRelease {
    fn id(&self) -> Option<&str> {
        None
    }

    fn name(&self) -> &Name {
        &self.version
    }
}

impl FromStr for SoftwareRelease {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(SoftwareRelease {
            version: input.to_string(),
            ..Default::default()
        })
    }
}
