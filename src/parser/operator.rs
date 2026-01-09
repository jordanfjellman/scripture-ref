use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Op {
    And,
    BookOf,
    ChapterOf,
    Following,
    Select,
    Through,
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Op::And => "and",
            Op::BookOf => "book of",
            Op::ChapterOf => "chapter of",
            Op::Following => "following",
            Op::Select => "select",
            Op::Through => "through",
        };
        write!(f, "{}", text)
    }
}
