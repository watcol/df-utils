//! JSON(JSON5) Parser
use df_utils::{Parser, Json5Parser, PrintConfig};
use std::io::{stdin, stdout, Read};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).expect("Failed to Read stdin");
    let value = Json5Parser::parse(&s).unwrap_or_else(|e| {
        println!("{}", e);
        std::process::exit(1);
    });
    value.print(&mut stdout(), &PrintConfig::new()).expect("Failed to write to stdout");
}
