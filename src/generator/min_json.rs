/// Minified JSON Generator.
pub struct MinJsonGenerator;

use crate::{Value, Generator};

impl Generator for MinJsonGenerator {
    type Err = ();
    fn generate(value: &Value) -> Result<String, Self::Err> {
        Ok(inner_generate(value))
    }
}

fn inner_generate(value: &Value) -> String {
    String::new()
}
