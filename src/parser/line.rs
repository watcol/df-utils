//! Line Parser

use crate::{Parser, Value};
use crate::datetime::*;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LineParser {
    root: String,
    delimiter: String,
    equal: String,
}

impl Default for LineParser {
    fn default() -> Self {
        Self {
            root: "$".to_string(),
            delimiter: ".".to_string(),
            equal: " = ".to_string(),
        }
    }
}

impl LineParser {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn root<S: Into<String>>(&mut self, root: S) -> &mut Self {
        self.root = root.into();
        self
    }

    pub fn delimiter<S: Into<String>>(&mut self, delimiter: S) -> &mut Self {
        self.delimiter = delimiter.into();
        self
    }

    pub fn equal<S: Into<String>>(&mut self, equal: S) -> &mut Self {
        self.equal = equal.into();
        self
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Path {
    Array(usize),
    Map(String),
}

#[derive(Clone, Debug)]
pub struct Item {
    path: Vec<Path>,
    value: Value,
}

peg::parser! {grammar printer() for str {
    use peg::ParseLiteral;
    use std::iter::FromIterator;
    use std::convert::TryFrom;

    pub rule printer(config: &LineParser) -> Vec<Item>
        = i:(item(config)**"\n") "\n"? { i }

    rule item(config: &LineParser) -> Item
        = root(config)
          delim(config)
          path:(path(config)**delim(config))
          equal(config)
          value:value()
          { Item { path, value } }

    rule root(config: &LineParser)
        = ##parse_string_literal(&config.root)

    rule delim(config: &LineParser)
        = ##parse_string_literal(&config.delimiter)

    rule equal(config: &LineParser)
        = ##parse_string_literal(&config.equal)

    rule path(config: &LineParser) -> Path
        = i:array() { Path::Array(i) }
        / s:map(config) { Path::Map(s) }

    rule array() -> usize
        = i:$(['1'..='9']['0'..='9']* / "0") {? i.parse().or(Err("array")) }

    rule map(config: &LineParser) -> String
        = s:$((!delim(config) !equal(config) [_])+) { s.to_string() }

    rule value() -> Value
        = "null"       { Value::Null }
        / "true"       { Value::Boolean(true) }
        / "false"      { Value::Boolean(false) }
        / "NaN"        { Value::Float(f64::NAN) }
        / "inf"        { Value::Float(f64::INFINITY) }
        / "-inf"       { Value::Float(f64::NEG_INFINITY) }
        / "[]"         { Value::Array(Vec::new()) }
        / "{}"         { Value::Map(HashMap::new()) }
        / d:datetime() { Value::DateTime(d) }
        / n:number()   { n }
        / s:string()   { Value::String(s) }

    rule string() -> String
        = "\"" c:ch()* "\"" { String::from_iter(c) }

    rule ch() -> char
        = c:$([^ '"' | '\\']) { c.chars().next().unwrap() }
        / "\\" e:escape() { e }

    rule escape() -> char
        = "\"" { '"' }
        / "'"  { '\'' }
        / "\\" { '\\' }
        / "0"  { '\0' }
        / "n"  { '\n' }
        / "r"  { '\r' }
        / "t"  { '\t' }
        / "u{" h:hex() "}" {?
            char::try_from(
                u32::from_str_radix(h, 16).or(Err("hex"))?
            ).or(Err("escape"))
        }

    rule hex() -> &'input str
        = $(['0'..='9' | 'a'..='f' | 'A'..='F']+)

    // rule number() -> Value
    //     = d:digits() f:fraction()? {?
    //         Ok(if let Some(f) = f {
    //             Value::Float([d, f].concat().parse().or(Err("float"))?)
    //         } else {
    //             Value::Int(d.parse().or(Err("int"))?)
    //         })
    //     }

    rule number() -> Value
        = d:digits() {? Ok(Value::Int(d.parse().or(Err("int"))?)) }

    rule digits() -> &'input str
        = $(("-"/"") (['1'..='9']['0'..='9']* / "0"))

    rule fraction() -> &'input str
        = $("." ['0'..='9']+)

    rule datetime() -> DateTime
        = d:date() t:(("T" / " ") t:time() { t })? o:offset()? {?
            DateTime::new(Some(d), t, o).or(Err("datetime"))
        }
        / t:time() {? DateTime::new(None, Some(t), None).or(Err("datetime")) }

    rule date() -> Date
        = y:digits4() "-" m:digits2() "-" d:digits2() {? Date::new(y, m, d).or(Err("date")) }

    rule time() -> Time
        = h:digits2() ":" m:digits2() ":" s:digits2() f:time_fraction()? {?
            Time::new(h, m, s, f.unwrap_or(0)).or(Err("time"))
        }

    rule offset() -> Offset
        = "Z" {? Offset::new(0, 0).or(Err("offset")) }
        / s:sign() h:digits2() ":" m:digits2() {?
            Offset::new(s * (h as i8), m).or(Err("offset"))
        }

    rule sign() -> i8
        = "+" {  1 }
        / "-" { -1 }

    rule digits4() -> u16
        = y:$(['0'..='9']*<4>) {? y.parse().or(Err("year")) }

    rule digits2() -> u8
        = m:$(['0'..='9']*<2>) {? m.parse().or(Err("month")) }

    rule time_fraction() -> u32
        = "." f:$(['0'..='9']*<1,9>) {?
            format!("{:0<9}", f).parse().or(Err("time_fraction"))
        }
}}

impl Value {
    fn append_item(&mut self, item: &mut Item) {
        if item.path.is_empty() {
            *self = item.value.clone();
            return;
        }

        let path = item.path.drain(0..1).next().unwrap();
        let child = match path {
            Path::Array(i) => {
                if !matches!(self, Value::Array(_)) {
                    *self = Value::Array(Vec::new());
                }
                if let Value::Array(v) = self {
                    if v.len() == i {
                        v.insert(i, Value::Null);
                    }
                    &mut v[i]
                } else {
                    unreachable!()
                }
            }
            Path::Map(k) => {
                if !matches!(self, Value::Map(_)) {
                    *self = Value::Map(HashMap::new());
                }
                if let Value::Map(m) = self {
                    if !m.contains_key(&k) {
                        m.insert(k.clone(), Value::Null);
                    }
                    m.get_mut(&k).unwrap()
                } else {
                    unreachable!()
                }
            }
        };

        child.append_item(item)
    }
}

impl Parser for LineParser {
    type Err = peg::error::ParseError<peg::str::LineCol>;
    fn parse(&self, s: &str) -> Result<Value, Self::Err> {
        let mut items = printer::printer(s, self)?;

        let mut value = Value::Null;
        for i in items.iter_mut() {
            value.append_item(i);
        }

        Ok(value)
    }
}
