//! Generator Implementations
mod min_json;

pub use min_json::MinJsonGenerator;

use crate::Value;
use std::io;

/// The unified interface for generate data format.
pub trait Generator {
    fn generate<W: io::Write>(buf: &mut W, value: &Value) -> io::Result<()>;
}
