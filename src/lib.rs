//! Simple CLI Parser for Data Formats
mod json;
mod print;

pub use json::{JsonParser, Json5Parser};
pub use print::PrintConfig;

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

impl Value {
    pub fn print<W: std::io::Write>(&self, buf: &mut W, config: &PrintConfig) -> std::io::Result<()> {
        print::print(buf, self, config)
    }

    pub fn parse(s: &str, config: &PrintConfig) -> Result<Self, peg::error::ParseError<peg::str::LineCol>> {
        print::deprint(s, config)
    }
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
