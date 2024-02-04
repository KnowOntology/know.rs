// This is free and unencumbered software released into the public domain.

use super::ThingLike;
use crate::prelude::*;

pub trait FileLike: ThingLike {
    fn size(&self) -> u64;
}

#[derive(Debug, Clone, Default, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct File {
    pub name: Name,

    pub size: u64,
}

impl ThingLike for File {
    fn id(&self) -> Option<&str> {
        None
    }

    fn name(&self) -> &Name {
        &self.name
    }
}

impl FileLike for File {
    fn size(&self) -> u64 {
        self.size
    }
}
