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
    Ok(())
}
