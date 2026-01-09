use std::fmt;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Token {
    Book(crate::bvc::BookSeries),
    Colon,
    Comma,
    Dash,
    // https://www.chicagomanualofstyle.org/qanda/data/faq/topics/Documentation/faq0361.html
    // F, // next
    FF, // all following
    Number(u8),
    Period,
    SemiColon,
    Subverse,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Book(b) => write!(f, "BOOK {b}"),
            Token::Comma => write!(f, "COMMA null"),
            Token::Colon => write!(f, "COLON null"),
            Token::Dash => write!(f, "DASH null"),
            Token::FF => write!(f, "FF null"),
            Token::Number(n) => write!(f, "NUMBER {n}"),
            Token::Period => write!(f, "PERIOD null"),
            Token::SemiColon => write!(f, "SEMICOLON null"),
            Token::Subverse => write!(f, "SUBVERSE null"),
        }
    }
}
