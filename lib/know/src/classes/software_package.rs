// This is free and unencumbered software released into the public domain.

use super::ThingLike;
use crate::prelude::*;
use std::{rc::Rc, str::FromStr};

/// See: https://en.wikipedia.org/wiki/Software_package
#[derive(Debug, Clone, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SoftwarePackage {
    pub name: Name,

    #[cfg_attr(feature = "serde", serde(default))]
    pub version: String,

    pub link: Option<IRI>,
}

impl ThingLike for SoftwarePackage {
    fn id(&self) -> Option<&str> {
        None
    }

    fn name(&self) -> Option<&Name> {
        Some(&self.name)
    }
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

pub type SoftwarePackageRef = Rc<SoftwarePackage>;

impl ThingLike for SoftwarePackageRef {
    fn id(&self) -> Option<&str> {
        None
    }

    fn name(&self) -> Option<&Name> {
        Some(&self.name)
    }
}
