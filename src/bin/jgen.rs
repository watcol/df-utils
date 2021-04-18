//! JSON Generator
use df_utils::{Value, PrintConfig, Generator, MinJsonGenerator};
use std::io::{Read, stdin, stdout};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).expect("Failed to read");
    let value = Value::parse(&s, &PrintConfig::new()).unwrap_or_else(|e| {
        println!("{}", e);
        std::process::exit(1);
    });
    MinJsonGenerator::generate(&mut stdout(), &value).expect("Failed to write");
}
