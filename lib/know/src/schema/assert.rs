// This is free and unencumbered software released into the public domain.

use super::Literal;

#[derive(Debug)]
pub enum Subject {
    Thing(String),
}

#[derive(Debug)]
pub enum Predicate {
    Know(String),
}

#[derive(Debug)]
pub enum Object {
    Thing(String),
    Literal(Literal),
}

#[derive(Debug)]
pub struct Assertion {
    pub predicate: Predicate,
    pub subject: Subject,
    pub object: Object,
}
