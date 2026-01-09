use std::iter::Peekable;

use crate::{bvc::Book, lexer::Token};
use binding_power::{BindingPower, infix_binding_power};
use miette::miette;
use operator::Op;
use token_tree::Node;

use crate::Lexer;

pub mod binding_power;
// pub mod context;
pub mod operand;
pub mod operator;
// pub mod state_machine;
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
            Token::Book(b) => {
                let book = Book::try_from((None, &b)).map_err(|e| miette!("{e}"))?;
                self.lexer.next();
                let right = self.parse_expression(BindingPower::Book as u8)?;
                Node::InBook(book, Box::new(right))
            }
            Token::Colon => todo!(),
            Token::Comma => todo!(),
            Token::Dash => todo!(),
            Token::FF => todo!(),
            Token::Number(n) => {
                let peeked = self.lexer.next_if(|v| matches!(v, Ok(Token::Book(_))));
                if let Some(Ok(Token::Book(b))) = peeked {
                    let book = Book::try_from((Some(n), &b)).map_err(|e| miette!("{e}"))?;
                    let right = self.parse_expression(BindingPower::Book as u8)?;
                    Node::InBook(book, Box::new(right))
                } else {
                    Node::Number(n)
                }
            }
            Token::Period => todo!(),
            Token::SemiColon => todo!(),
            Token::Subverse => todo!(),
        };

        loop {
            let token = self.lexer.peek();
            let operator = match token {
                None => break,
                Some(Ok(Token::Number(_))) => {
                    // let chapter = *n;
                    // let rhs = self.parse_expression(min_bp)?;
                    // return Ok(Node::InChapter(chapter, Box::new(rhs)));
                    let rhs = self.parse_expression(min_bp);
                    return rhs;
                }
                Some(Ok(Token::Comma)) => Op::Select,
                Some(Ok(Token::Colon)) => Op::ChapterOf,
                Some(Ok(Token::Dash)) => Op::Through,
                Some(Ok(Token::SemiColon)) => Op::And,
                Some(_) => {
                    todo!("handle other tokens")
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
                Op::BookOf => todo!(),
                Op::Following => todo!(),
                Op::Select => Node::Select(Box::new(lhs), Box::new(rhs)),
                Op::Through => Node::Through(Box::new(lhs), Box::new(rhs)),
            };
            continue;
        }

        Ok(lhs)
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
