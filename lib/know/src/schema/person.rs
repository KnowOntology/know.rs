// This is free and unencumbered software released into the public domain.

use std::str::FromStr;

#[cfg(feature = "serde")]
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default, PartialEq, PartialOrd)]
pub struct Person {
    pub name: String,
}

impl FromStr for Person {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Person {
            name: input.to_string(),
        })
    }
}
