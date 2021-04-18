//! Parser Implementations
mod json;
mod json5;
mod line;

pub use json::JsonParser;
pub use json5::Json5Parser;
pub use line::LineParser;

use crate::Value;

/// The unified interface for parsing data format.
pub trait Parser {
    type Err;
    fn parse(&self, s: &str) -> Result<Value, Self::Err>;
}
