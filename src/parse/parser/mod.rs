use std::iter::Peekable;

use crate::{
    bvc::{ChapterNumber, VerseNumber},
    parse::lexer::{Lexer, Token},
};
use binding_power::{BindingPower, infix_binding_power};
use operator::Op;
use token_tree::Node;

pub mod binding_power;
pub mod context;
pub mod operator;
pub mod token_tree;

pub struct Parser<'de> {
    lexer: Peekable<Lexer<'de>>,
}

impl<'de> Parser<'de> {
    pub fn new(input: &'de str) -> Self {
        Self {
            lexer: Lexer::new(input).peekable(),
        }
    }

    pub fn parse(&mut self) -> Result<Node, miette::Error> {
        self.parse_expression(BindingPower::Minimum as u8)
    }

    fn current(&mut self) -> Result<Option<Token>, miette::Error> {
        self.lexer
            .next()
            .transpose()
            .map_err(|e| e.wrap_err("parsing current token"))
    }

    fn parse_expression(&mut self, min_bp: u8) -> Result<Node, miette::Error> {
        let current = self.current()?;
        let current = match current {
            Some(t) => t,
            None => return Ok(Node::Nil),
        };

        let mut lhs = match current {
            Token::Book(book) => {
                // Book token now contains the complete Book (including numbered books like "1 Kings")
                let right = self.parse_expression(BindingPower::Book as u8)?;
                Node::InBook(book, Box::new(right))
            }
            Token::Number(n) => Node::Number(n),
            Token::VersePart(p) => Node::VersePart(p),
            Token::Colon => unimplemented!(),
            Token::Comma => unimplemented!(),
            Token::Dash => unimplemented!(),
            Token::FF => unimplemented!(),
            Token::Period => unimplemented!(),
            Token::SemiColon => unimplemented!(),
        };

        loop {
            let token = self.lexer.peek();
            let operator = match token {
                None => break,
                Some(Ok(Token::Number(_))) => {
                    let rhs = self.parse_expression(min_bp);
                    return rhs;
                }
                Some(Ok(Token::VersePart(p))) => {
                    let part = *p;
                    if let Node::Number(verse_num) = lhs {
                        self.lexer.next();
                        lhs = Node::InVerse(
                            VerseNumber::try_from(verse_num).map_err(|e| miette::miette!("{e}"))?,
                            Box::new(Node::VersePart(part)),
                        );
                        continue;
                    } else {
                        break;
                    }
                }
                Some(Ok(Token::Comma)) => Op::Select,
                Some(Ok(Token::Colon)) => Op::ChapterOf,
                Some(Ok(Token::Dash)) => Op::Through,
                Some(Ok(Token::SemiColon)) => Op::And,
                Some(Ok(Token::FF)) => Op::Following,
                Some(token) => {
                    todo!("handle other tokens {token:?}");
                }
            };

            let (l_bp, r_bp) = infix_binding_power(operator);
            if l_bp < min_bp {
                break;
            }
            self.lexer.next();
            let rhs = self.parse_expression(r_bp)?;
            lhs = match operator {
                Op::And => Node::And(Box::new(lhs), Box::new(rhs)),
                Op::ChapterOf => {
                    let chapter = lhs.try_into().map_err(|e| {
                        miette::miette! {
                            "{e}"
                        }
                    })?;
                    Node::InChapter(chapter, Box::new(rhs))
                }
                Op::BookOf => unimplemented!(),
                Op::Following => Node::Following(Box::new(lhs)),
                Op::Select => Node::Select(Box::new(lhs), Box::new(rhs)),
                Op::Through => Node::Through(Box::new(lhs), Box::new(rhs)),
                Op::PartOf => unimplemented!(),
            };
            continue;
        }

        Ok(lhs)
    }
}

impl TryFrom<Node> for ChapterNumber {
    type Error = String;

    fn try_from(value: Node) -> Result<Self, Self::Error> {
        match value {
            Node::InChapter(chapter, _) => Ok(chapter),
            Node::Number(n) => Ok(ChapterNumber::try_from(n)?),
            _ => Err("not a chapter number".to_string()),
        }
    }
}

impl TryFrom<Node> for VerseNumber {
    type Error = String;

    fn try_from(value: Node) -> Result<Self, Self::Error> {
        match value {
            Node::InVerse(verse, _) => Ok(verse),
            Node::Number(n) => Ok(VerseNumber::try_from(n)?),
            _ => Err("not a verse number".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{bvc::Book, parser::Parser, parser::token_tree::Node};

    #[test]
    fn parse_book_of_operator() {
        let test_cases = vec![
            (
                "1 kings",
                Book::FirstKings,
                "with named with a numbered prefix",
            ),
            ("Psalms", Book::Psalms, "with a single word name"),
            // (
            //     "song of solomon",
            //     Book::SongOfSolomon,
            //     "with a multiple word name",
            // ),
        ];
        for (reference, book, case_desc) in test_cases {
            let mut parser = Parser::new(reference);
            let parsed = parser
                .parse()
                .unwrap_or_else(|_| panic!("failed to handle books {}", case_desc));
            assert_eq!(parsed, Node::InBook(book, Box::new(Node::Nil)));
        }
    }

    // #[test]
    // fn parses_following() {
    //     let mut parser = Parser::new("8ff");
    //     let parsed = parser.parse().expect("should have parsed");
    //     assert_eq!("(following 8 nil)", format!("{}", parsed));
    // }
    //
    // #[test]
    // fn following_has_higher_power_than_and() {
    //     let mut parser = Parser::new("1;8ff");
    //     let parsed = parser.parse().expect("should have parsed");
    //     assert_eq!("(and 1 (following 8 nil)", format!("{}", parsed));
    // }
}
