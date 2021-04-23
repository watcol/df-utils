//! Simple CLI Parser for Data Formats
pub mod generator;
pub mod parser;
pub mod datetime;

#[cfg(feature = "bin")]
pub mod io;

pub use generator::Generator;
pub use parser::Parser;

use std::collections::HashMap;
use datetime::DateTime;

/// Represents the Parsed value.
#[derive(Clone, Debug)]
pub enum Value {
    Null,
    Int(i128),
    Float(f64),
    Boolean(bool),
    String(String),
    DateTime(DateTime),
    Array(Vec<Value>),
    Map(HashMap<String, Value>),
}
