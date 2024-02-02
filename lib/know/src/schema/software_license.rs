// This is free and unencumbered software released into the public domain.

/// See: https://en.wikipedia.org/wiki/Software_license
/// See: https://spdx.org/licenses/
#[cfg(feature = "serde")]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, Default, PartialEq, PartialOrd)]
pub enum SoftwareLicense {
    Unspecified,
    #[default]
    Unlicense,
    Other(String),
}
