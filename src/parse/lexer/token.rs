use std::fmt;

#[derive(Debug, PartialEq, Copy, Clone)]
pub(crate) enum Token {
    Book(crate::bvc::Book),
    Colon,
    Comma,
    Dash,
    // https://www.chicagomanualofstyle.org/qanda/data/faq/topics/Documentation/faq0361.html
    // F, // next
    FF, // all following
    Number(u8),
    Period,
    SemiColon,
    VersePart(u8),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Book(b) => write!(f, "{b}"),
            Token::Colon => write!(f, ":"),
            Token::Comma => write!(f, ","),
            Token::Dash => write!(f, "-"),
            Token::FF => write!(f, "ff"),
            Token::Number(n) => write!(f, "{n}"),
            Token::Period => write!(f, "."),
            Token::SemiColon => write!(f, ";"),
            Token::VersePart(p) => write!(f, "{}", (*p as char).to_string()),
        }
    }
}
