// This is free and unencumbered software released into the public domain.

use super::prelude::*;
use std::str::FromStr;

#[cfg(feature = "serde")]
use serde_with::serde_as;

#[cfg_attr(feature = "serde", cfg_eval::cfg_eval, serde_as)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default, PartialEq, PartialOrd)]
pub struct Person {
    pub name: Name,

    pub birthdate: Option<Date>,

    #[cfg_attr(
        feature = "serde",
        serde(alias = "email", default),
        serde_as(as = "serde_with::OneOrMany<_>")
    )]
    pub emails: Vec<Email>,
}

impl Person {
    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn birthdate(&self) -> Option<&Date> {
        self.birthdate.as_ref()
    }

    pub fn email(&self) -> Option<&Email> {
        self.emails.first()
    }

    pub fn emails(&self) -> &Vec<Email> {
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
