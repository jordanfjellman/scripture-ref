use std::fmt;

use crate::bvc::{Book, ChapterNumber, VerseNumber};

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    And(Box<Node>, Box<Node>),
    #[allow(unused)]
    Book(Book),
    Following(Box<Node>),
    InBook(Book, Box<Node>),
    InChapter(ChapterNumber, Box<Node>),
    InVerse(VerseNumber, Box<Node>),
    Through(Box<Node>, Box<Node>),
    Select(Box<Node>, Box<Node>),
    Number(u8),
    Nil,
    VersePart(u8),
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::And(left, right) => write!(f, "{} and(;) {}", left, right),
            Node::Book(book) => write!(f, "{}", book),
            Node::Following(verse) => write!(f, "all following {}", verse),
            Node::InBook(book, node) => write!(f, "{} in the book of {}", node, book),
            Node::InChapter(chapter, node) => write!(f, "verse {} in chapter {}", node, chapter),
            Node::InVerse(verse, node) => write!(f, "verse part {} in verse {}", node, verse),
            Node::Through(initial, end) => write!(f, "{} through {}", initial, end),
            Node::Select(left, right) => write!(f, "select {} and(,) {}", left, right),
            Node::Number(number) => write!(f, "{}", number),
            Node::Nil => write!(f, "nil"),
            Node::VersePart(part) => write!(f, "{}", part),
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
