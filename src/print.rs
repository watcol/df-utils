//! Print Value
use std::io;
use crate::Value;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PrintConfig {
    root: String,
    delimiter: String,
    equal: String,
}

impl Default for PrintConfig {
    fn default() -> Self {
        Self {
            root: "$".to_string(),
            delimiter: ".".to_string(),
            equal: " = ".to_string(),
        }
    }
}

impl PrintConfig {
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

pub fn print<W: io::Write>(buf: &mut W, value: &Value, config: &PrintConfig) -> io::Result<()> {
    print_inner(buf, value, config, &config.root)
}

fn print_inner<W: io::Write>(buf: &mut W, value: &Value, config: &PrintConfig, loc: &str) -> io::Result<()> {
    match value {
        Value::Null => writeln!(buf, "{}{}null", loc, config.equal)?,
        Value::Boolean(b) => writeln!(buf, "{}{}{}", loc, config.equal, b)?,
        Value::Int(i) => writeln!(buf, "{}{}{}", loc, config.equal, i)?,
        Value::Float(f) => writeln!(buf, "{}{}{}", loc, config.equal, f)?,
        Value::String(s) => writeln!(buf, "{}{}{:?}", loc, config.equal, s)?,
        Value::Array(vs) => {
            for (i, v) in vs.iter().enumerate() {
                print_inner(buf, v, config, &[loc, &config.delimiter, &i.to_string()].concat())?
            }
        }
        Value::Map(vs) => {
            for (k, v) in vs.iter() {
                print_inner(buf, v, config, &[loc, &config.delimiter, &k].concat())?
            }
        }
    }
    Ok(())
}
