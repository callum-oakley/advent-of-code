use std::{
    fmt::Display,
    iter::Peekable,
    ops::Index,
    rc::Rc,
    str::{Chars, FromStr},
};

use anyhow::{Error, Result, bail, ensure};

#[derive(Clone, PartialEq, Eq)]
enum Inner {
    Nil,
    Pair(Value, Value),
    Int(i32),
}

/// S-expressions delimited by square brackets, with `,` treated as whitespace.
#[derive(Clone, PartialEq, Eq)]
pub struct Value(Rc<Inner>);

impl Value {
    pub fn nil() -> Self {
        Value(Rc::new(Inner::Nil))
    }

    pub fn cons(head: &Self, tail: &Self) -> Self {
        assert!(tail.is_list());
        Value(Rc::new(Inner::Pair(head.clone(), tail.clone())))
    }

    pub fn int(int: i32) -> Self {
        Value(Rc::new(Inner::Int(int)))
    }

    pub fn is_nil(&self) -> bool {
        matches!(self.inner(), Inner::Nil)
    }

    pub fn is_list(&self) -> bool {
        matches!(self.inner(), Inner::Nil | Inner::Pair(_, _))
    }

    pub fn is_int(&self) -> bool {
        matches!(self.inner(), Inner::Int(_))
    }

    pub fn head(&self) -> &Self {
        match self.inner() {
            Inner::Pair(head, _) => head,
            Inner::Nil => panic!("empty list"),
            Inner::Int(_) => panic!("not a list"),
        }
    }

    pub fn tail(&self) -> &Self {
        match self.inner() {
            Inner::Pair(_, tail) => tail,
            Inner::Nil => panic!("empty list"),
            Inner::Int(_) => panic!("not a list"),
        }
    }

    pub fn unint(&self) -> i32 {
        match self.inner() {
            Inner::Int(int) => *int,
            _ => panic!("not an int"),
        }
    }

    fn inner(&self) -> &Inner {
        self.0.as_ref()
    }
}

impl Index<usize> for Value {
    type Output = Value;

    fn index(&self, i: usize) -> &Self::Output {
        if i == 0 {
            self.head()
        } else {
            &self.tail()[i - 1]
        }
    }
}

impl FromStr for Value {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        fn parse(chars: &mut Peekable<Chars>) -> Result<Value> {
            while chars.peek().is_some_and(|&c| c.is_whitespace() || c == ',') {
                chars.next();
            }
            match chars.peek() {
                Some('[') => {
                    chars.next();
                    let list = parse_items(chars)?;
                    ensure!(matches!(chars.next(), Some(']')));
                    Ok(list)
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

        fn parse_items(chars: &mut Peekable<Chars>) -> Result<Value> {
            while chars.peek().is_some_and(|&c| c.is_whitespace() || c == ',') {
                chars.next();
            }
            match chars.peek() {
                Some(']') => Ok(Value::nil()),
                Some(_) => {
                    let head = parse(chars)?;
                    Ok(Value::cons(&head, &parse_items(chars)?))
                }
                None => bail!("unexpected EOF"),
            }
        }

        parse(&mut s.chars().peekable())
    }
}

impl<'a, I: IntoIterator<Item = &'a Value>> From<I> for Value {
    fn from(value: I) -> Self {
        fn go<'a, I: Iterator<Item = &'a Value>>(mut iter: I) -> Value {
            match iter.next() {
                Some(head) => Value::cons(head, &go(iter)),
                None => Value::nil(),
            }
        }
        go(value.into_iter())
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_int() {
            write!(f, "{}", self.unint())?;
        } else {
            let mut value = self;
            write!(f, "[")?;
            let mut first = true;
            while !value.is_nil() {
                if !first {
                    write!(f, " ")?;
                }
                write!(f, "{}", value.head())?;
                value = value.tail();
                first = false;
            }
            write!(f, "]")?;
        }
        Ok(())
    }
}
