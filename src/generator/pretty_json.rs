/// Formatted JSON Generator.
pub struct PrettyJsonGenerator {
    indent: usize,
}

impl Default for PrettyJsonGenerator {
    fn default() -> Self {
        Self { indent: 4 }
    }
}

impl PrettyJsonGenerator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn indent(&mut self, indent: usize) -> &mut Self {
        self.indent = indent;
        self
    }
}

use crate::{Generator, Value};
use std::io::{self, Write};

impl Generator for PrettyJsonGenerator {
    fn generate<W: Write>(&self, buf: &mut W, value: &Value) -> io::Result<()> {
        inner_generate(buf, value, self.indent, 1)?;
        writeln!(buf)
    }
}

fn inner_generate<W: Write>(
    buf: &mut W,
    value: &Value,
    ind_size: usize,
    ind: usize,
) -> io::Result<()> {
    match value {
        Value::Null => write!(buf, "null")?,
        Value::Boolean(b) => write!(buf, "{}", b)?,
        Value::Int(i) => write!(buf, "{}", i)?,
        Value::Float(f) if *f == f64::INFINITY => write!(buf, "Infinity")?,
        Value::Float(f) if *f == f64::NEG_INFINITY => write!(buf, "-Infinity")?,
        Value::Float(f) => write!(buf, "{}", f)?,
        Value::String(s) => string(buf, s)?,
        Value::DateTime(d) => write!(buf, "\"{}\"", d)?,
        Value::Array(vs) => {
            writeln!(buf, "[")?;
            for (i, v) in vs.iter().enumerate() {
                if i != 0 {
                    writeln!(buf, ",")?;
                }
                write!(buf, "{}", " ".repeat(ind_size * ind))?;
                inner_generate(buf, v, ind_size, ind + 1)?;
            }
            write!(buf, "\n{}]", " ".repeat(ind_size * (ind - 1)))?;
        }
        Value::Map(m) => {
            writeln!(buf, "{{")?;
            for (i, (k, v)) in m.iter().enumerate() {
                if i != 0 {
                    writeln!(buf, ",")?;
                }
                write!(buf, "{}", " ".repeat(ind_size * ind))?;
                string(buf, k)?;
                write!(buf, ": ")?;
                inner_generate(buf, v, ind_size, ind + 1)?;
            }
            write!(buf, "\n{}}}", " ".repeat(ind_size * (ind - 1)))?;
        }
    }
    Ok(())
}

fn string<W: Write>(buf: &mut W, s: &str) -> io::Result<()> {
    write!(buf, "\"")?;
    for c in s.chars() {
        match c {
            '"' => write!(buf, "\\\"")?,
            '\\' => write!(buf, "\\\\")?,
            '\x08' => write!(buf, "\\b")?,
            '\x0c' => write!(buf, "\\f")?,
            '\n' => write!(buf, "\\n")?,
            '\r' => write!(buf, "\\r")?,
            '\t' => write!(buf, "\\t")?,
            c if c < ' ' => write!(buf, "\\u{:04x}", c as u8)?,
            c => write!(buf, "{}", c)?,
        }
    }
    write!(buf, "\"")?;
    Ok(())
}
