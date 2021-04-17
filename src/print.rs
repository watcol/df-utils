//! Print Value
use std::io;
use crate::Value;

pub fn print<W: io::Write>(buf: &mut W, value: &Value) -> io::Result<()> {
    print_inner(buf, value, "$")
}

fn print_inner<W: io::Write>(buf: &mut W, value: &Value, loc: &str) -> io::Result<()> {
    match value {
        Value::Null => writeln!(buf, "{} = null", loc)?,
        Value::Boolean(b) => writeln!(buf, "{} = {}", loc, b)?,
        Value::Int(i) => writeln!(buf, "{} = {}", loc, i)?,
        Value::Float(f) => writeln!(buf, "{} = {}", loc, f)?,
        Value::String(s) => writeln!(buf, "{} = {:?}", loc, s)?,
        Value::Array(vs) => {
            for (i, v) in vs.iter().enumerate() {
                print_inner(buf, v, &[loc, ".", &i.to_string()].concat())?
            }
        }
        Value::Map(vs) => {
            for (k, v) in vs.iter() {
                print_inner(buf, v, &[loc, ".", &k].concat())?
            }
        }
    }
    Ok(())
}
