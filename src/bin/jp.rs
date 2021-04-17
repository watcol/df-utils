//! JSON(JSON5) Parser
use df_utils::{Value, Parser, JsonParser};
use std::io::{stdin, Read};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).expect("Failed to Read stdin");
    let value = JsonParser::parse(&s).unwrap_or_else(|e| {
        println!("{}", e);
        Value::Null
    });
    println!("{:?}", value);
}
