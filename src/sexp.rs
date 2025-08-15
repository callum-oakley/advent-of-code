use std::{
    fmt::Display,
    iter::Peekable,
    rc::Rc,
    str::{Chars, FromStr},
};

use anyhow::{Error, Result, bail, ensure};

#[derive(Clone, PartialEq, Eq)]
pub enum Inner {
    Int(i64),
    Vec(Vec<Value>),
}

/// S-expressions delimited by square brackets, with `,` treated as whitespace.
#[derive(Clone, PartialEq, Eq)]
pub struct Value(Rc<Inner>);

impl Value {
    pub fn int(int: i64) -> Self {
        Value(Rc::new(Inner::Int(int)))
    }

    pub fn vec(values: Vec<Value>) -> Self {
        Value(Rc::new(Inner::Vec(values)))
    }

    pub fn as_inner(&self) -> &Inner {
        self.0.as_ref()
    }

    pub fn as_int(&self) -> Option<i64> {
        match self.as_inner() {
            Inner::Int(int) => Some(*int),
            Inner::Vec(_) => None,
        }
    }
}

impl FromStr for Value {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        fn go(chars: &mut Peekable<Chars>) -> Result<Value> {
            while chars.peek().is_some_and(|&c| c.is_whitespace() || c == ',') {
                chars.next();
            }
            match chars.peek() {
                Some('[') => {
                    chars.next();
                    let mut values = Vec::new();
                    while chars.peek().is_some_and(|&c| c != ']') {
                        values.push(go(chars)?);
                    }
                    ensure!(matches!(chars.next(), Some(']')));
                    Ok(Value::vec(values))
                }
                Some('-' | '+' | '0'..='9') => {
                    let mut int = String::new();
                    int.push(chars.next().unwrap());
                    while chars.peek().is_some_and(char::is_ascii_digit) {
                        int.push(chars.next().unwrap());
                    }
                    Ok(Value::int(int.parse()?))
                }
                Some(c) => bail!("unexpected character {c}"),
                None => bail!("unexpected EOF"),
            }
        }

        go(&mut s.chars().peekable())
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.as_inner() {
            Inner::Int(int) => {
                write!(f, "{int}")?;
            }
            Inner::Vec(vec) => {
                write!(f, "[")?;
                let mut it = vec.iter();
                if let Some(value) = it.next() {
                    write!(f, "{value}")?;
                }
                for value in it {
                    write!(f, " {value}")?;
                }
                write!(f, "]")?;
            }
        }
        Ok(())
    }
}
