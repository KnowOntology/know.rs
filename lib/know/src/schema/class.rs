// This is free and unencumbered software released into the public domain.

use super::{literal::LangStrings, property::Property};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Class {
    pub id: String,
    pub subclass_of: Option<String>,
    pub glyph: Option<String>,
    pub label: Option<LangStrings>,
    pub comment: Option<LangStrings>,
    pub see_also: Option<LangStrings>,
    pub properties: Option<HashMap<String, Property>>,
}

impl Class {
    #[allow(unused)]
    pub fn new(
        id: String,
        subclass_of: Option<String>,
        glyph: Option<String>,
        label: Option<LangStrings>,
        comment: Option<LangStrings>,
        see_also: Option<LangStrings>,
        properties: Option<HashMap<String, Property>>,
    ) -> Self {
        Self {
            id,
            subclass_of,
            glyph,
            label,
            comment,
            see_also,
            properties,
        }
    }
}
