//! Parser Implementations
mod json;
mod json5;

pub use json::JsonParser;
pub use json5::Json5Parser;

use crate::Value;

/// The unified interface for parsing data format.
pub trait Parser {
    type Err;
    fn parse(s: &str) -> Result<Value, Self::Err>;
}

