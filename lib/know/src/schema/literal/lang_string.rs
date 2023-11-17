// This is free and unencumbered software released into the public domain.

use super::lang_tag::LangTag;

#[derive(Debug, Default)]
pub struct LangString {
    pub lang: LangTag,
    pub string: String,
}

impl LangString {
    pub fn new(lang: LangTag, string: String) -> Self {
        Self { lang, string }
    }
}
