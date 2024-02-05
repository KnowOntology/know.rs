// This is free and unencumbered software released into the public domain.

use crate::{classes::*, prelude::*};

pub trait ThingLike {
    fn id(&self) -> Option<&str>;
    fn name(&self) -> &Name;
}

#[derive(Debug, Clone, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Thing {
    Event(Event),
    File(File),
    Group(Group),
    Link(Link),
    Person(Person),
    Place(Place),
    Project(Project),
    SoftwarePackage(SoftwarePackage),
    SoftwareProject(SoftwareProject),
    SoftwareRelease(SoftwareRelease),
}

impl ThingLike for Thing {
    fn id(&self) -> Option<&str> {
        match self {
            Thing::Event(e) => e.id(),
            Thing::File(f) => f.id(),
            Thing::Group(g) => g.id(),
            Thing::Link(l) => l.id(),
            Thing::Person(p) => p.id(),
            Thing::Place(p) => p.id(),
            Thing::Project(p) => p.id(),
            Thing::SoftwarePackage(p) => p.id(),
            Thing::SoftwareProject(p) => p.id(),
            Thing::SoftwareRelease(p) => p.id(),
        }
    }

    fn name(&self) -> &Name {
        match self {
            Thing::Event(e) => e.name(),
            Thing::File(f) => f.name(),
            Thing::Group(g) => g.name(),
            Thing::Link(l) => l.name(),
            Thing::Person(p) => p.name(),
            Thing::Place(p) => p.name(),
            Thing::Project(p) => p.name(),
            Thing::SoftwarePackage(p) => p.name(),
            Thing::SoftwareProject(p) => p.name(),
            Thing::SoftwareRelease(p) => p.name(),
        }
    }
}
