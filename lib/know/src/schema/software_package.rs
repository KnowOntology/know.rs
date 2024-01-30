// This is free and unencumbered software released into the public domain.

/// See: https://en.wikipedia.org/wiki/Software_package
#[cfg(feature = "serde")]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Default, PartialEq, PartialOrd)]
pub struct SoftwarePackage {
    pub name: String,
    pub version: String,
    pub link: Option<String>,
}
