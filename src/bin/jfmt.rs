//! JSON Formatter

use df_utils::{Parser, Generator, Json5Parser, PrettyJsonGenerator};
use std::io::{Read, stdin, stdout};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).expect("Failed to read");
    let value = Json5Parser::parse(&s).unwrap_or_else(|e| {
        println!("{}", e);
        std::process::exit(1);
    });
    PrettyJsonGenerator::generate(&mut stdout(), &value).expect("Failed to write.");
}
