//! Simple CLI Parser for Data Formats
mod generator;
mod parser;
mod print;

#[cfg(feature = "bin")]
pub mod io;

pub use generator::{Generator, MinJsonGenerator, PrettyJsonGenerator};
pub use parser::{Json5Parser, JsonParser, Parser};
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
    Map(HashMap<String, Value>),
}

impl Value {
    pub fn print<W: std::io::Write>(
        &self,
        buf: &mut W,
        config: &PrintConfig,
    ) -> std::io::Result<()> {
        print::print(buf, self, config)
    }

    pub fn parse(
        s: &str,
        config: &PrintConfig,
    ) -> Result<Self, peg::error::ParseError<peg::str::LineCol>> {
        print::deprint(s, config)
    }
}
