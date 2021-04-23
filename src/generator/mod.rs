//! Generator Implementations
mod line;
mod min_json;
mod pretty_json;
mod toml;

pub use line::LineGenerator;
pub use min_json::MinJsonGenerator;
pub use pretty_json::PrettyJsonGenerator;
pub use toml::TomlGenerator;

use crate::Value;
use std::io;
use std::path::Path;

/// The unified interface for generate data format.
pub trait Generator {
    fn generate<W: io::Write>(&self, buf: &mut W, value: &Value) -> io::Result<()>;

    #[cfg(feature = "bin")]
    fn write_path<P: AsRef<Path>>(&self, path: Option<P>, value: &Value) -> io::Result<()> {
        self.generate(&mut crate::io::Output::from_path(path)?, value)
    }
}
