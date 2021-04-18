//! Print Value
use crate::Value;
use std::collections::HashMap;
use std::io;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PrintConfig {
    root: String,
    delimiter: String,
    equal: String,
}

impl Default for PrintConfig {
    fn default() -> Self {
        Self {
            root: "$".to_string(),
            delimiter: ".".to_string(),
            equal: " = ".to_string(),
        }
    }
}

impl PrintConfig {
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

pub fn print<W: io::Write>(buf: &mut W, value: &Value, config: &PrintConfig) -> io::Result<()> {
    print_inner(buf, value, config, &config.root)
}

fn print_inner<W: io::Write>(
    buf: &mut W,
    value: &Value,
    config: &PrintConfig,
    loc: &str,
) -> io::Result<()> {
    match value {
        Value::Null => writeln!(buf, "{}{}null", loc, config.equal)?,
        Value::Boolean(b) => writeln!(buf, "{}{}{}", loc, config.equal, b)?,
        Value::Int(i) => writeln!(buf, "{}{}{}", loc, config.equal, i)?,
        Value::Float(f) => writeln!(buf, "{}{}{}", loc, config.equal, f)?,
        Value::String(s) => writeln!(buf, "{}{}{:?}", loc, config.equal, s)?,
        Value::Array(vs) if vs.is_empty() => writeln!(buf, "{}{}[]", loc, config.equal)?,
        Value::Array(vs) => {
            for (i, v) in vs.iter().enumerate() {
                print_inner(
                    buf,
                    v,
                    config,
                    &[loc, &config.delimiter, &i.to_string()].concat(),
                )?
            }
        }
        Value::Map(vs) if vs.is_empty() => writeln!(buf, "{}{}{{}}", loc, config.equal)?,
        Value::Map(vs) => {
            for (k, v) in vs.iter() {
                print_inner(buf, v, config, &[loc, &config.delimiter, &k].concat())?
            }
        }
    }
    Ok(())
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

    pub rule printer(config: &PrintConfig) -> Vec<Item>
        = i:(item(config)**"\n") "\n"? { i }

    rule item(config: &PrintConfig) -> Item
        = root(config)
          delim(config)
          path:(path(config)**delim(config))
          equal(config)
          value:value()
          { Item { path, value } }

    rule root(config: &PrintConfig)
        = ##parse_string_literal(&config.root)

    rule delim(config: &PrintConfig)
        = ##parse_string_literal(&config.delimiter)

    rule equal(config: &PrintConfig)
        = ##parse_string_literal(&config.equal)

    rule path(config: &PrintConfig) -> Path
        = i:array() { Path::Array(i) }
        / s:map(config) { Path::Map(s) }

    rule array() -> usize
        = i:$(['1'..='9']['0'..='9']* / "0") {? i.parse().or(Err("array")) }

    rule map(config: &PrintConfig) -> String
        = s:$((!delim(config) !equal(config) [_])+) { s.to_string() }

    rule value() -> Value
        = "null"     { Value::Null }
        / "true"     { Value::Boolean(true) }
        / "false"    { Value::Boolean(false) }
        / "NaN"      { Value::Float(f64::NAN) }
        / "inf"      { Value::Float(f64::INFINITY) }
        / "-inf"     { Value::Float(f64::NEG_INFINITY) }
        / "[]"       { Value::Array(Vec::new()) }
        / "{}"       { Value::Map(HashMap::new()) }
        / n:number() { n }
        / s:string() { Value::String(s) }

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

pub fn deprint(
    s: &str,
    config: &PrintConfig,
) -> Result<Value, peg::error::ParseError<peg::str::LineCol>> {
    let mut items = printer::printer(s, config)?;

    let mut value = Value::Null;
    for i in items.iter_mut() {
        value.append_item(i);
    }

    Ok(value)
}
