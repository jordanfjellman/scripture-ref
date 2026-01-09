use std::fmt;

use crate::bvc::Book;

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    And(Box<Node>, Box<Node>),
    Book(Book),
    InBook(Book, Box<Node>),
    InChapter(u8, Box<Node>),
    Through(Box<Node>, Box<Node>),
    Select(Box<Node>, Box<Node>),
    Number(u8),
    Nil,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::And(left, right) => write!(f, "{} and(;) {}", left, right),
            Node::Book(book) => write!(f, "{}", book),
            Node::InBook(book, node) => write!(f, "{} in the book of {}", node, book),
            Node::InChapter(chapter, node) => write!(f, "verse {} in chapter {}", node, chapter),
            Node::Through(initial, end) => write!(f, "{} through {}", initial, end),
            Node::Select(left, right) => write!(f, "select {} and(,) {}", left, right),
            Node::Number(number) => write!(f, "{}", number),
            Node::Nil => write!(f, "nil"),
        }
    }
}

impl TryInto<u8> for Node {
    // TODO: "not a number" error
    type Error = String;

    fn try_into(self) -> Result<u8, Self::Error> {
        match self {
            Node::Number(n) => Ok(n),
            other => Err(format!("{} is not a number", other)),
        }
    }
}
