//! JSON Generator
use df_utils::{Value, PrintConfig};
use std::io::{Read, stdin};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).expect("Failed to read");
    println!("{}", s);
    println!("{:?}", Value::parse(&s, &PrintConfig::new()));
}
