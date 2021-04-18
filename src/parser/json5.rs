//! JSON5 Parser
use crate::{Parser, Value};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::iter::FromIterator;
use unicode_categories::UnicodeCategories;

peg::parser! {grammar json5_parser() for str {
    pub rule json5() -> Value
        = elem()

    rule _ = whitespace()*
    rule __ = "\n" / "\r" / "\u{2028}" / "\u{2029}" / ""
    rule whitespace() = "//" [^'\n' | '\r' | '\u{2028}' | '\u{2029}']* __
           / "/*" [^'*']* "*/"
           / [c if c.is_whitespace() || c == '\u{feff}']

    rule elem() -> Value
        = _ v:value() _ { v }

    rule value() -> Value
        = object()
        / array()
        / string()
        / number()
        / bool()
        / null()

    rule object() -> Value
        = "{" _ m:(member() ** ",") _ ","? _ "}" { Value::Map(HashMap::from_iter(m)) }

    rule member() -> (String, Value)
        = _ s:(ident() / string_()) _ ":" e:elem() { (s, e) }

    rule ident() -> String
        = s:ident_start() p:ident_part()* { let mut string = String::from_iter(p); string.insert(0, s); string }

    rule ident_start() -> char
        = c:$([c if c.is_letter_uppercase() ||
                    c.is_letter_lowercase() ||
                    c.is_letter_titlecase() ||
                    c.is_letter_modifier()  ||
                    c.is_letter_other()     ||
                    c == '$' || c == '_'
              ]) { c.chars().next().unwrap() }
        / "u" h:$(hex()*<4>) {? char::try_from(u32::from_str_radix(h, 16).or(Err("hexchar"))?).or(Err("escape")) }

    rule ident_part() -> char
        = ident_start()
        / c:$([c if c.is_mark_nonspacing()        ||
                    c.is_mark_spacing_combining() ||
                    c.is_number_decimal_digit()   ||
                    c.is_punctuation_connector()  ||
                    c == '\u{200c}' || c == '\u{200d}'
              ]) { c.chars().next().unwrap() }

    rule array() -> Value
        = "[" _ e:(elem() ** ",") _ ","? _ "]" { Value::Array(e) }

    rule bool() -> Value
        = "true" { Value::Boolean(true) }
        / "false" { Value::Boolean(false) }

    rule null() -> Value
        = "null" { Value::Null }

    rule string() -> Value
        = s:string_() { Value::String(s) }

    rule string_() -> String
        = "\"" s:double_char()* "\"" { String::from_iter(s.into_iter().filter_map(|c| c)) }
        / "'" s:single_char()* "'" { String::from_iter(s.into_iter().filter_map(|c| c)) }

    rule double_char() -> Option<char>
        = c:$([^ '"' | '\\' | '\n' | '\r']) { Some(c.chars().next().unwrap()) }
        / "\\" c:(
            e:escape() { Some(e) }
            / __ { None }
        ) { c }

    rule single_char() -> Option<char>
        = c:$([^ '\'' | '\\' | '\n' | '\r']) { Some(c.chars().next().unwrap()) }
        / "\\" c:(
            e:escape() { Some(e) }
            / __ { None }
        ) { c }

    rule escape() -> char
        = "\"" { '"' }
        / "\'" { '\'' }
        / "\\" { '\\' }
        / "/"  { '/' }
        / "b"  { '\x08' }
        / "f"  { '\x0c' }
        / "v"  { '\x0b' }
        / "n"  { '\n' }
        / "r"  { '\r' }
        / "t"  { '\t' }
        / "x" h:$(hex()*<2>) {? Ok(u8::from_str_radix(h, 16).or(Err("hexchar"))? as char) }
        / "u" h:$(hex()*<4>) {? char::try_from(u32::from_str_radix(h, 16).or(Err("hexchar"))?).or(Err("escape")) }

    rule number() -> Value
        = "-" v:unsigned() {
            match v {
                Value::Int(i) => Value::Int(-i),
                Value::Float(f) => Value::Float(-f),
                _ => unreachable!(),
            }
        }
        / ("+"/"") v:unsigned() { v }

    rule unsigned() -> Value
        = "Infinity" { Value::Float(f64::INFINITY) }
        / "NaN" { Value::Float(f64::NAN) }
        / "0x" h:$(hex()+) {? Ok(Value::Int(i128::from_str_radix(h, 16).or(Err("hexint"))?)) }
        / d:$("." ['0'..='9']+ exponent()) {? Ok(Value::Float(d.parse().or(Err("float"))?)) }
        / i:digits() d:$(fraction() exponent()) {?
            Ok(if d.is_empty() {
                Value::Int(i.parse().or(Err("int"))?)
            } else {
                Value::Float([i, d].concat().parse().or(Err("float"))?)
            })
        }

    rule exponent() -> &'input str
        = $(("e"/"E") ("+"/"-"/"") digits() / "")

    rule fraction() -> &'input str
        = $("." ['0'..='9']+ / "")

    rule hex() -> &'input str
        = $(['0'..='9' | 'a'..='f' | 'A'..='F'])

    rule digits() -> &'input str
        = $("0" / ['1'..='9']['0'..='9']*)
}}

#[derive(Copy, Clone, Debug)]
pub struct Json5Parser;

impl Parser for Json5Parser {
    type Err = peg::error::ParseError<peg::str::LineCol>;
    fn parse(s: &str) -> Result<Value, Self::Err> {
        json5_parser::json5(s)
    }
}
