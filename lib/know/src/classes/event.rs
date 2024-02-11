// This is free and unencumbered software released into the public domain.

use super::ThingLike;
use crate::prelude::*;
use std::{
    fmt::{Debug, Display, Formatter},
    rc::Rc,
};

pub trait EventLike: ThingLike {}

#[derive(Debug, Clone, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Event {
    pub name: Name,
}

impl ThingLike for Event {
    fn id(&self) -> Option<&str> {
        None
    }

    fn name(&self) -> &Name {
        &self.name
    }
}

impl EventLike for Event {}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct EventRef(pub Rc<Event>);

impl ThingLike for EventRef {
    fn id(&self) -> Option<&str> {
        None
    }

    fn name(&self) -> &Name {
        self.0.name()
    }
}

impl Debug for EventRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = &mut f.debug_struct("EventRef");
        if !self.0.name.is_empty() {
            result = result.field("name", &self.0.name);
        }
        result.finish()
    }
}

impl Display for EventRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
