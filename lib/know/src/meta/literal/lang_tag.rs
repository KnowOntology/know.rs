// This is free and unencumbered software released into the public domain.

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub enum LangTag {
    #[serde(rename = "en", alias = "en-US", alias = "en-GB")]
    #[default]
    English,
    Other(String),
}
