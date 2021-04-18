/// Minified JSON Generator.
pub struct MinJsonGenerator;

use crate::{Value, Generator};
use std::io::{self, Write};

impl Generator for MinJsonGenerator {
    fn generate<W: Write>(buf: &mut W, value: &Value) -> io::Result<()> {
        inner_generate(buf, value)
    }
}

fn inner_generate<W: Write>(buf: &mut W, value: &Value) -> io::Result<()> {
    match value {
        Value::Null => write!(buf, "null")?,
        Value::Boolean(b) => write!(buf, "{}", b)?,
        Value::Int(i) => write!(buf, "{}", i)?,
        Value::Float(f) if *f == f64::INFINITY => write!(buf, "Infinity")?,
        Value::Float(f) if *f == f64::NEG_INFINITY => write!(buf, "-Infinity")?,
        Value::Float(f) => write!(buf, "{}", f)?,
        Value::String(s) => write!(buf, "{:?}", s)?,
        Value::Array(vs) => {
            write!(buf, "[")?;
            for (i, v) in vs.iter().enumerate() {
                if i != 0 { write!(buf, ",")?; }
                inner_generate(buf, v)?;
            }
            write!(buf, "]")?;
        }
        Value::Map(m) => {
            write!(buf, "{{")?;
            for (i, (k, v)) in m.iter().enumerate() {
                if i != 0 { write!(buf, ",")?; }
                write!(buf, "{:?}:", k)?;
                inner_generate(buf, v)?;
            }
            write!(buf, "}}")?;
        }
    }
    Ok(())
}
