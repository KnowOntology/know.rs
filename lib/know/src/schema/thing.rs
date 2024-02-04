// This is free and unencumbered software released into the public domain.

use super::prelude::*;

pub trait ThingLike {
    fn id(&self) -> Option<&str>;
    fn name(&self) -> &Name;
}
