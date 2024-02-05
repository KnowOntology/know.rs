// This is free and unencumbered software released into the public domain.

use super::ThingLike;
use crate::prelude::*;
use std::{
    fmt::{Display, Formatter},
    rc::Rc,
    str::FromStr,
};

#[cfg(feature = "serde")]
use serde_with::serde_as;

pub trait PersonLike: ThingLike {
    fn birthdate(&self) -> Option<&Date>;
    fn parents(&self) -> Vec<PersonRef>;
    fn father(&self) -> Option<&PersonRef>;
    fn mother(&self) -> Option<&PersonRef>;
    fn siblings(&self) -> &Vec<PersonRef>;
    fn spouse(&self) -> Option<&PersonRef>;
    fn children(&self) -> &Vec<PersonRef>;
    fn colleagues(&self) -> &Vec<PersonRef>;
    fn knows(&self) -> &Vec<PersonRef>;
    fn email(&self) -> Option<&Email>;
    fn emails(&self) -> &Vec<Email>;
}

#[derive(Debug, Clone, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval, serde_as)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Person {
    pub name: Name,

    pub birthdate: Option<Date>,

    pub father: Option<PersonRef>,

    pub mother: Option<PersonRef>,

    #[cfg_attr(
        feature = "serde",
        serde(
            default,
            alias = "sibling",
            alias = "brothers",
            alias = "brother",
            alias = "sisters",
            alias = "sister"
        ),
        serde_as(as = "serde_with::OneOrMany<_>")
    )]
    pub siblings: Vec<PersonRef>,

    #[cfg_attr(feature = "serde", serde(default, alias = "husband", alias = "wife"))]
    pub spouse: Option<PersonRef>,

    #[cfg_attr(
        feature = "serde",
        serde(default, alias = "child"),
        serde_as(as = "serde_with::OneOrMany<_>")
    )]
    pub children: Vec<PersonRef>,

    #[cfg_attr(
        feature = "serde",
        serde(default, alias = "colleague"),
        serde_as(as = "serde_with::OneOrMany<_>")
    )]
    pub colleagues: Vec<PersonRef>,

    #[cfg_attr(
        feature = "serde",
        serde(default),
        serde_as(as = "serde_with::OneOrMany<_>")
    )]
    pub knows: Vec<PersonRef>,

    #[cfg_attr(
        feature = "serde",
        serde(default, alias = "email"),
        serde_as(as = "serde_with::OneOrMany<_>")
    )]
    pub emails: Vec<Email>,
}

impl ThingLike for Person {
    fn id(&self) -> Option<&str> {
        None
    }

    fn name(&self) -> &Name {
        &self.name
    }
}

impl PersonLike for Person {
    fn birthdate(&self) -> Option<&Date> {
        self.birthdate.as_ref()
    }

    fn parents(&self) -> Vec<PersonRef> {
        let mut result = vec![];
        if let Some(father) = self.father() {
            result.push(father.clone());
        }
        if let Some(mother) = self.mother() {
            result.push(mother.clone());
        }
        result
    }

    fn father(&self) -> Option<&PersonRef> {
        self.father.as_ref()
    }

    fn mother(&self) -> Option<&PersonRef> {
        self.mother.as_ref()
    }

    fn siblings(&self) -> &Vec<PersonRef> {
        self.siblings.as_ref()
    }

    fn spouse(&self) -> Option<&PersonRef> {
        self.spouse.as_ref()
    }

    fn children(&self) -> &Vec<PersonRef> {
        self.children.as_ref()
    }

    fn colleagues(&self) -> &Vec<PersonRef> {
        self.colleagues.as_ref()
    }

    fn knows(&self) -> &Vec<PersonRef> {
        self.knows.as_ref()
    }

    fn email(&self) -> Option<&Email> {
        self.emails.first()
    }

    fn emails(&self) -> &Vec<Email> {
        &self.emails
    }
}

impl FromStr for Person {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Person {
            name: input.to_string(),
            ..Default::default()
        })
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct PersonRef(Rc<Person>);

impl Display for PersonRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.name)
    }
}

impl FromStr for PersonRef {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Person::from_str(input).map(Rc::new).map(PersonRef)
    }
}
