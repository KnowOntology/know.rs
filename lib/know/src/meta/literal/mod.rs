// This is free and unencumbered software released into the public domain.

mod lang_string;
mod lang_strings;
mod lang_tag;

pub use lang_string::*;
pub use lang_strings::*;
pub use lang_tag::*;

#[derive(Debug)]
pub enum LiteralType {
    String,
    LangString,
}

#[derive(Debug)]
pub enum Literal {
    String(String),
    LangString(LangString),
}
