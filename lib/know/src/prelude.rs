// This is free and unencumbered software released into the public domain.

use iri_string::types::IriString;

#[cfg(feature = "chrono")]
pub type Date = chrono::NaiveDate;

#[cfg(not(feature = "chrono"))]
pub type Date = String;

pub type Email = String;

pub type IRI = IriString;

pub type Name = String;
