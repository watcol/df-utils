//! Simple CLI Parser for Data Formats
mod json;
pub use json::JsonParser;

use std::collections::HashMap;

/// Represents the Parsed value.
#[derive(Clone, Debug)]
pub enum Value {
    Null,
    Int(i128),
    Float(f64),
    Boolean(bool),
    String(String),
    Array(Vec<Value>),
    Map(HashMap<String, Value>)
}

/// The unified interface for parsing data format.
pub trait Parser {
    type Err;
    fn parse(s: &str) -> Result<Value, Self::Err>;
}

/// The unified interface for generate data format.
pub trait Generator {
    type Err;
    fn generate(value: &Value) -> Result<String, Self::Err>;
}
