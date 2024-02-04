// This is free and unencumbered software released into the public domain.

use std::str::FromStr;

#[cfg(feature = "serde")]
use serde_with::serde_as;

#[cfg_attr(feature = "serde", cfg_eval::cfg_eval, serde_as)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default, PartialEq, PartialOrd)]
pub struct Person {
    pub name: String,

    #[cfg_attr(
        feature = "serde",
        serde(alias = "email", default),
        serde_as(as = "serde_with::OneOrMany<_>")
    )]
    pub emails: Vec<String>,
}

impl Person {
    pub fn email(&self) -> Option<&String> {
        self.emails.first()
    }

    pub fn emails(&self) -> &Vec<String> {
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
