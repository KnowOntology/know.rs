// This is free and unencumbered software released into the public domain.

use crate::Error;
use know::classes::Thing;
use serde::Deserialize;
use std::io::Read;

pub struct Parser<'de>(serde_yaml::Deserializer<'de>);

impl<'de> Parser<'de> {
    pub fn from_reader<R>(input: R) -> Self
    where
        R: Read + 'de,
    {
        Parser(serde_yaml::Deserializer::from_reader(input))
    }
}

impl<'de> Iterator for Parser<'de> {
    type Item = Result<Thing, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.next().map(|result| Thing::deserialize(result)) {
            Some(Ok(thing)) => Some(Ok(thing)),
            Some(Err(err)) => Some(Err(err.into())),
            None => None,
        }
    }
}
