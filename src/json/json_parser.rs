//! JSON Parser
use std::convert::TryFrom;
use std::iter::FromIterator;
use crate::{Value, Parser};

peg::parser!{grammar json_parser() for str {
    pub rule json() -> Value
        = elem()

    rule _ = " " / "\n" / "\r" / "\t" / ""

    rule elem() -> Value
        = _ v:value() _ { v }

    rule value() -> Value
        = number()
        / string()

    rule string() -> Value
        = "\"" s:character()* "\"" { Value::String(String::from_iter(s)) }

    rule character() -> char
        = c:$([^ '"' | '\\']) { c.chars().next().unwrap() }
        / "\\" e:escape() { e }

    rule escape() -> char
        = "\"" { '"' }
        / "\\" { '\\' }
        / "/"  { '/' }
        / "b"  { '\x08' }
        / "f"  { '\x0c' }
        / "n"  { '\n' }
        / "r"  { '\r' }
        / "t"  { '\t' }
        / "u" h:hex() {? char::try_from(h).or(Err("hex")) }

    rule hex() -> u32
        = h:$(['0'..='9' | 'a'..='f' | 'A'..='F']*<4,4>) {? h.parse().or(Err("hex")) }

    rule number() -> Value
        = i:signed() d:$(fraction() exponent()) {?
            if d.is_empty() {
                i.parse().map(Value::Int).or(Err("int"))
            } else {
                [i, d].concat().parse().map(Value::Float).or(Err("float"))
            }
        }

    rule exponent() -> &'input str
        = $(("e"/"E") signed() / "")

    rule fraction() -> &'input str
        = $("." ['0'..='9']+ / "")

    rule signed() -> &'input str
        = $(("+" / "-" / "") digits())

    rule digits() -> &'input str
        = $("0" / ['1'..='9']['0'..='9']*)
}}

#[derive(Copy, Clone, Debug)]
pub struct JsonParser;

impl Parser for JsonParser {
    type Err = peg::error::ParseError<peg::str::LineCol>;
    fn parse(s: &str) -> Result<Value, Self::Err> {
        json_parser::json(s)
    }
}
