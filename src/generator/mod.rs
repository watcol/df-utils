//! Generator Implementations

use crate::Value;

/// The unified interface for generate data format.
pub trait Generator {
    type Err;
    fn generate(value: &Value) -> Result<String, Self::Err>;
}
