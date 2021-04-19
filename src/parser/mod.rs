//! Parser Implementations
mod json;
mod json5;
mod line;

pub use json::JsonParser;
pub use json5::Json5Parser;
pub use line::LineParser;

use crate::Value;
use std::error::Error;
use std::fmt;
use std::io;
use std::path::Path;

/// The error type for "read_from" function.
#[derive(Debug)]
pub enum CombinedError<E> {
    Io(io::Error),
    Parse(E),
}

impl<E: Error> Error for CombinedError<E> {}

impl<E: Error> fmt::Display for CombinedError<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "IO Error: {}", e),
            Self::Parse(e) => write!(f, "Parse Error: {}", e),
        }
    }
}

impl<E: Error> From<io::Error> for CombinedError<E> {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

/// The unified interface for parsing data format.
pub trait Parser {
    type Err: Error;
    fn parse(&self, s: &str) -> Result<Value, Self::Err>;

    fn read_from<R: io::Read>(&self, buf: &mut R) -> Result<Value, CombinedError<Self::Err>> {
        let mut s = String::new();
        buf.read_to_string(&mut s)?;
        self.parse(&s).map_err(CombinedError::Parse)
    }

    #[cfg(feature = "bin")]
    fn read_path<P: AsRef<Path>>(
        &self,
        path: Option<P>,
    ) -> Result<Value, CombinedError<Self::Err>> {
        self.read_from(&mut crate::io::Input::from_path(path)?)
    }
}
