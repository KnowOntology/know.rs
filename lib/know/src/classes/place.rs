// This is free and unencumbered software released into the public domain.

use super::ThingLike;
use crate::prelude::*;

pub trait PlaceLike: ThingLike {}

#[derive(Debug, Clone, Default, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Place {
    pub name: Name,
}

impl ThingLike for Place {
    fn id(&self) -> Option<&str> {
        None
    }

    fn name(&self) -> &Name {
        &self.name
    }
}

impl PlaceLike for Place {}
