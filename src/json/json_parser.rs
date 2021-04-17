//! JSON Parser
use crate::{Value, Parser};

peg::parser!{grammar json_parser() for str {
    pub rule json() -> Value
        = i:integer() { Value::Int(i) }

    rule integer() -> i64
        = n:$(['0'..='9']+) {? n.parse().or(Err("integer")) }
}}

#[derive(Copy, Clone, Debug)]
pub struct JsonParser;

impl Parser for JsonParser {
    type Err = peg::error::ParseError<peg::str::LineCol>;
    fn parse(s: &str) -> Result<Value, Self::Err> {
        json_parser::json(s)
    }
}
