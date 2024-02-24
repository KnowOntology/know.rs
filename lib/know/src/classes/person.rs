// This is free and unencumbered software released into the public domain.

use super::{EventRef, ThingLike};
use crate::prelude::*;
use std::{
    fmt::{Debug, Display, Formatter},
    rc::Rc,
    str::FromStr,
};

#[cfg(feature = "serde")]
use serde_with::serde_as;

pub trait PersonLike: ThingLike {
    fn nickname(&self) -> Option<&Name>;
    fn nicknames(&self) -> &Vec<Name>;
    fn age(&self) -> Option<Age>;
    fn birthdate(&self) -> Option<Date>;
    fn birth(&self) -> Option<&EventRef>;
    fn death(&self) -> Option<&EventRef>;
    fn parents(&self) -> Vec<PersonRef>;
    fn father(&self) -> Option<&PersonRef>;
    fn mother(&self) -> Option<&PersonRef>;
    fn siblings(&self) -> &Vec<PersonRef>;
    fn spouse(&self) -> Option<&PersonRef>;
    fn spouses(&self) -> &Vec<PersonRef>;
    fn partner(&self) -> Option<&PersonRef>;
    fn partners(&self) -> &Vec<PersonRef>;
    fn children(&self) -> &Vec<PersonRef>;
    fn colleagues(&self) -> &Vec<PersonRef>;
    fn knows(&self) -> &Vec<PersonRef>;
    fn email(&self) -> Option<&Email>;
    fn emails(&self) -> &Vec<Email>;
    fn phone(&self) -> Option<&Phone>;
    fn phones(&self) -> &Vec<Phone>;
}

#[derive(Debug, Clone, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", cfg_eval::cfg_eval, serde_as)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Person {
    pub name: Name,

    #[cfg_attr(
        feature = "serde",
        serde(default, alias = "nickname"),
        serde_as(as = "serde_with::OneOrMany<_>")
    )]
    pub nicknames: Vec<Name>,

    pub age: Option<Age>,

    pub birth: Option<EventRef>,

    pub death: Option<EventRef>,

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

    #[cfg_attr(
        feature = "serde",
        serde(default, alias = "spouse", alias = "husband", alias = "wife"),
        serde_as(as = "serde_with::OneOrMany<_>")
    )]
    pub spouses: Vec<PersonRef>,

    #[cfg_attr(
        feature = "serde",
        serde(default, alias = "partner", alias = "boyfriend", alias = "girlfriend"),
        serde_as(as = "serde_with::OneOrMany<_>")
    )]
    pub partners: Vec<PersonRef>,

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

    #[cfg_attr(
        feature = "serde",
        serde(default, alias = "phone"),
        serde_as(as = "serde_with::OneOrMany<_>")
    )]
    pub phones: Vec<Phone>,
}

impl ThingLike for Person {
    fn id(&self) -> Option<&str> {
        None
    }

    fn name(&self) -> Option<&Name> {
        Some(&self.name)
    }
}

impl PersonLike for Person {
    fn nickname(&self) -> Option<&Name> {
        self.nicknames.first()
    }

    fn nicknames(&self) -> &Vec<Name> {
        self.nicknames.as_ref()
    }

    fn age(&self) -> Option<Age> {
        self.age // TODO: calculate from self.birthdate
    }

    fn birthdate(&self) -> Option<Date> {
        match self.birth {
            Some(ref event) => event.0.date,
            None => None,
        }
    }

    fn birth(&self) -> Option<&EventRef> {
        self.birth.as_ref()
    }

    fn death(&self) -> Option<&EventRef> {
        self.death.as_ref()
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
        self.spouses.first()
    }

    fn spouses(&self) -> &Vec<PersonRef> {
        self.spouses.as_ref()
    }

    fn partner(&self) -> Option<&PersonRef> {
        self.partners.first()
    }

    fn partners(&self) -> &Vec<PersonRef> {
        self.partners.as_ref()
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
        self.emails.as_ref()
    }

    fn phone(&self) -> Option<&Phone> {
        self.phones.first()
    }

    fn phones(&self) -> &Vec<Phone> {
        self.phones.as_ref()
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
#[derive(Clone, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct PersonRef(pub Rc<Person>);

impl ThingLike for PersonRef {
    fn id(&self) -> Option<&str> {
        None
    }

    fn name(&self) -> Option<&Name> {
        self.0.name()
    }
}

impl Debug for PersonRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = &mut f.debug_struct("PersonRef");
        if !self.0.name.is_empty() {
            result = result.field("name", &self.0.name);
        }
        result = match self.0.nicknames.len() {
            0 => result,
            1 => result.field("nickname", &self.0.nicknames[0]),
            _ => result.field("nicknames", &self.0.nicknames),
        };
        result = match self.0.emails.len() {
            0 => result,
            1 => result.field("email", &self.0.emails[0]),
            _ => result.field("emails", &self.0.emails),
        };
        result.finish()
    }
}

impl Display for PersonRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match (self.id(), self.name(), self.0.email()) {
            (Some(id), Some(name), Some(email)) => write!(f, "{} <{}> (#{})", name, email, id),
            (Some(id), Some(name), None) => write!(f, "{} (#{})", name, id),
            (Some(id), None, Some(email)) => write!(f, "<{}> (#{})", email, id),
            (Some(id), None, None) => write!(f, "#{}", id),
            (None, Some(name), Some(email)) => write!(f, "{} <{}>", name, email),
            (None, Some(name), None) => write!(f, "{}", name),
            (None, None, Some(email)) => write!(f, "<{}>", email),
            (None, None, None) => write!(f, "↪Person"),
        }
    }
}

impl FromStr for PersonRef {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Person::from_str(input).map(Rc::new).map(PersonRef)
    }
}
