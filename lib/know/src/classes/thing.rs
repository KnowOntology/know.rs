// This is free and unencumbered software released into the public domain.

use crate::prelude::*;

pub trait ThingLike {
    fn id(&self) -> Option<&str>;
    fn name(&self) -> &Name;
}
