//! TOML Parser Implementation

use crate::{Value, Parser};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct TomlParser;

impl Parser for TomlParser {
    type Err = peg::error::ParseError<peg::str::LineCol>;
    fn parse(&self, _s: &str) -> Result<Value, Self::Err> {
        Ok(Value::Null)
    }
}
