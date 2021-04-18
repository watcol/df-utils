/// Formatted JSON Generator.
pub struct PrettyJsonGenerator;

use crate::{Value, Generator};
use std::io::{self, Write};

impl Generator for PrettyJsonGenerator {
    fn generate<W: Write>(buf: &mut W, value: &Value) -> io::Result<()> {
        inner_generate(buf, value, 1)?;
        writeln!(buf)
    }
}

fn inner_generate<W: Write>(buf: &mut W, value: &Value, ind: usize) -> io::Result<()> {
    match value {
        Value::Null => write!(buf, "null")?,
        Value::Boolean(b) => write!(buf, "{}", b)?,
        Value::Int(i) => write!(buf, "{}", i)?,
        Value::Float(f) if *f == f64::INFINITY => write!(buf, "Infinity")?,
        Value::Float(f) if *f == f64::NEG_INFINITY => write!(buf, "-Infinity")?,
        Value::Float(f) => write!(buf, "{}", f)?,
        Value::String(s) => string(buf, s)?,
        Value::Array(vs) => {
            writeln!(buf, "[")?;
            for (i, v) in vs.iter().enumerate() {
                if i != 0 { writeln!(buf, ",")?; }
                write!(buf, "{}", "    ".repeat(ind))?;
                inner_generate(buf, v, ind+1)?;
            }
            write!(buf, "\n{}]", "    ".repeat(ind-1))?;
        }
        Value::Map(m) => {
            writeln!(buf, "{{")?;
            for (i, (k, v)) in m.iter().enumerate() {
                if i != 0 { writeln!(buf, ",")?; }
                write!(buf, "{}", "    ".repeat(ind))?;
                string(buf, k)?;
                write!(buf, ": ")?;
                inner_generate(buf, v, ind+1)?;
            }
            write!(buf, "\n{}}}", "    ".repeat(ind-1))?;
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
