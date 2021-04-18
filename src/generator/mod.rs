//! Generator Implementations
mod min_json;

pub use min_json::MinJsonGenerator;

use crate::Value;

/// The unified interface for generate data format.
pub trait Generator {
    type Err;
    fn generate(value: &Value) -> Result<String, Self::Err>;
}
