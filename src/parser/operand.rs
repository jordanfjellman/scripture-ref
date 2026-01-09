use std::fmt;

use crate::bvc::Book;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Atom {
    Book(Book),
    Number(u8),
    Nil,
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Atom::Book(b) => write!(f, "{b}"),
            Atom::Number(n) => write!(f, "{n}"),
            Atom::Nil => write!(f, "nil"),
        }
    }
}
