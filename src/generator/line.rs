//! Line Generator

use crate::{Generator, Value};
use std::io;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LineGenerator {
    root: String,
    delimiter: String,
    equal: String,
}

impl Default for LineGenerator {
    fn default() -> Self {
        Self {
            root: "$".to_string(),
            delimiter: ".".to_string(),
            equal: " = ".to_string(),
        }
    }
}

impl LineGenerator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn root<S: Into<String>>(&mut self, root: S) -> &mut Self {
        self.root = root.into();
        self
    }

    pub fn delimiter<S: Into<String>>(&mut self, delimiter: S) -> &mut Self {
        self.delimiter = delimiter.into();
        self
    }

    pub fn equal<S: Into<String>>(&mut self, equal: S) -> &mut Self {
        self.equal = equal.into();
        self
    }
}

impl Generator for LineGenerator {
    fn generate<W: io::Write>(&self, buf: &mut W, value: &Value) -> io::Result<()> {
        inner(buf, value, self, &self.root)
    }
}

fn inner<W: io::Write>(
    buf: &mut W,
    value: &Value,
    config: &LineGenerator,
    loc: &str,
) -> io::Result<()> {
    match value {
        Value::Null => writeln!(buf, "{}{}null", loc, config.equal)?,
        Value::Boolean(b) => writeln!(buf, "{}{}{}", loc, config.equal, b)?,
        Value::Int(i) => writeln!(buf, "{}{}{}", loc, config.equal, i)?,
        Value::Float(f) => writeln!(buf, "{}{}{}", loc, config.equal, f)?,
        Value::String(s) => writeln!(buf, "{}{}{:?}", loc, config.equal, s)?,
        Value::Array(vs) if vs.is_empty() => writeln!(buf, "{}{}[]", loc, config.equal)?,
        Value::Array(vs) => {
            for (i, v) in vs.iter().enumerate() {
                inner(
                    buf,
                    v,
                    config,
                    &[loc, &config.delimiter, &i.to_string()].concat(),
                )?
            }
        }
        Value::Map(vs) if vs.is_empty() => writeln!(buf, "{}{}{{}}", loc, config.equal)?,
        Value::Map(vs) => {
            for (k, v) in vs.iter() {
                inner(buf, v, config, &[loc, &config.delimiter, &k].concat())?
            }
        }
    }
    Ok(())
}
