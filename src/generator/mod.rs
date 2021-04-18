//! Generator Implementations
mod line;
mod min_json;
mod pretty_json;

pub use line::LineGenerator;
pub use min_json::MinJsonGenerator;
pub use pretty_json::PrettyJsonGenerator;

use crate::Value;
use std::io;

/// The unified interface for generate data format.
pub trait Generator {
    fn generate<W: io::Write>(&self, buf: &mut W, value: &Value) -> io::Result<()>;
}
