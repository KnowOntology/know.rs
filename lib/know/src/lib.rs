// This is free and unencumbered software released into the public domain.

mod error;
mod feature;

#[cfg(feature = "serde")]
pub mod schema;

pub use error::*;
pub use feature::*;
