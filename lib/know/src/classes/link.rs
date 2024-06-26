// This is free and unencumbered software released into the public domain.

use super::ThingLike;
use crate::prelude::*;

pub trait LinkLike: ThingLike {
    fn url(&self) -> &IRI;
}

#[derive(Debug, Clone, /*Default,*/ Eq, Hash, PartialEq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Link {
    pub name: Name,

    pub url: IRI,
}

impl ThingLike for Link {
    fn id(&self) -> Option<&str> {
        Some(self.url.as_str())
    }

    fn name(&self) -> Option<&Name> {
        Some(&self.name)
    }
}

impl LinkLike for Link {
    fn url(&self) -> &IRI {
        &self.url
    }
}
