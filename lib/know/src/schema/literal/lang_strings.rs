// This is free and unencumbered software released into the public domain.

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct LangStrings {
    pub en: String,
}

impl LangStrings {
    pub fn new() -> Self {
        Self::default()
    }
}
