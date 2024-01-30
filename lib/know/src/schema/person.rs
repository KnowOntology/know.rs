// This is free and unencumbered software released into the public domain.

#[cfg(feature = "serde")]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Default, PartialEq, PartialOrd)]
pub struct Person {
    pub name: String,
}
