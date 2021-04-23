//!TOML Generator

use crate::{Value, Generator};
use std::io::Write;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct TomlGenerator;

impl Generator for TomlGenerator {
    fn generate<W: Write>(&self, _buf: &mut W, _value: &Value) -> std::io::Result<()> {
        Ok(())
    }
}
