#![allow(unused)]

use crate::bvc::Book;

pub mod token;

pub use token::Token;

#[derive(thiserror::Error, miette::Diagnostic, Debug)]
#[error("Unexpected token '{token}'")]
struct UnexpectedToken {
    #[source_code]
    src: String,

    token: char,

    #[label = "this input character"]
    err_span: miette::SourceSpan,
}

pub(crate) struct Lexer<'de> {
    original: &'de str,
    rest: &'de str,
    current_byte: usize,
    peeked: Option<Result<Token, miette::Error>>,
}

enum LeadToken {
    Identifer,
    Number,
}

impl<'de> Lexer<'de> {
    pub fn new(input: &'de str) -> Self {
        Self {
            original: input,
            rest: input,
            current_byte: 0,
            peeked: None,
        }
    }

    pub fn peek(&mut self) -> Option<&Result<Token, miette::Error>> {
        if self.peeked.is_some() {
            return self.peeked.as_ref();
        }
        self.peeked = self.next();
        self.peeked.as_ref()
    }
}

impl<'de> Iterator for Lexer<'de> {
    type Item = Result<Token, miette::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.peeked.take() {
            return Some(next);
        }

        loop {
            let mut chars = self.rest.chars();
            let c = chars.next()?;
            let c_onwards = self.rest;
            if c.is_whitespace() {
                let before_trim_len = self.rest.len();
                self.rest = c_onwards.trim_start();
                let after_trim_len = self.rest.len();
                self.current_byte += before_trim_len - after_trim_len;
                continue;
            } else {
                self.current_byte += c.len_utf8();
                self.rest = chars.as_str();
            }

            let lead = match c {
                ':' => return Some(Ok(Token::Colon)),
                ',' => return Some(Ok(Token::Comma)),
                '-' => return Some(Ok(Token::Dash)),
                ';' => return Some(Ok(Token::SemiColon)),
                '0'..='9' => LeadToken::Number,
                'a'..='z' | 'A'..='Z' => LeadToken::Identifer,
                c if c.is_whitespace() => unreachable!("handled before match"),
                c => {
                    let c_pos = self.current_byte - c.len_utf8();
                    return Some(Err(UnexpectedToken {
                        src: self.original.to_string(),
                        token: c,
                        err_span: (c_pos, c.len_utf8()).into(),
                    }
                    .into()));
                }
            };

            break match lead {
                // book, ff, or subverse
                LeadToken::Identifer => {
                    // Extract potential book name (up to first digit after whitespace, or punctuation)
                    let potential_book = extract_potential_book_name(c_onwards);

                    // Try parsing as a book (handles "Song of Songs", "Psalms", etc.)
                    if !potential_book.is_empty() {
                        if let Ok((book, bytes_consumed)) = Book::parse(potential_book) {
                            self.rest = &c_onwards[bytes_consumed..];
                            self.current_byte += bytes_consumed - c.len_utf8();
                            return Some(Ok(Token::Book(book)));
                        }
                    }

                    // Not a book - check for "ff" or other identifiers
                    let first_non_identifier = c_onwards
                        .find(|c| !matches!(c, 'a'..='z' | 'A'..='Z'))
                        .unwrap_or_else(|| c_onwards.len());
                    let literal = &c_onwards[..first_non_identifier];
                    let bytes_from_chars = literal.len() - c.len_utf8() + 1;
                    self.rest = &c_onwards[bytes_from_chars..];

                    let token = match literal {
                        "ff" => Ok(Token::FF),
                        // if 'a'..'z' => Ok(TokenKind::Subverse),
                        l => {
                            Err(miette::miette! {
                                labels = vec![miette::LabeledSpan::at(self.current_byte-1..self.current_byte + bytes_from_chars-1, "these literal characters")],
                                "not a valid book: {l}"
                            }
                            .with_source_code(self.original.to_string()))
                        }
                    };

                    self.current_byte += bytes_from_chars - 1;

                    Some(token)
                }
                LeadToken::Number => {
                    // Extract potential book name (for numbered books like "1 Kings")
                    let potential_book = extract_potential_book_name(c_onwards);

                    // Try parsing as a numbered book
                    if !potential_book.is_empty() {
                        if let Ok((book, bytes_consumed)) = Book::parse(potential_book) {
                            self.rest = &c_onwards[bytes_consumed..];
                            self.current_byte += bytes_consumed - c.len_utf8();
                            return Some(Ok(Token::Book(book)));
                        }
                    }

                    // Fall back to parsing just the number
                    let first_non_digit = c_onwards
                        .find(|c| !matches!(c, '0'..='9'))
                        .unwrap_or_else(|| c_onwards.len());
                    let digits = &c_onwards[..first_non_digit];
                    let bytes_from_digits = digits.len() - c.len_utf8() + 1;
                    self.rest = &c_onwards[bytes_from_digits..];
                    let n = match digits.parse() {
                        Ok(n) => n,
                        Err(_) => todo!(),
                    };

                    return Some(Ok(Token::Number(n)));
                }
            };
        }
    }
}

fn extract_potential_book_name(s: &str) -> &str {
    let bytes = s.as_bytes();
    let mut i = 0;
    let mut last_word_end = 0;
    let mut in_word = false;

    while i < bytes.len() {
        let b = bytes[i];

        if matches!(b, b':' | b',' | b'-' | b';') {
            break;
        }

        if b.is_ascii_digit() && last_word_end > 0 {
            if i > 0 && bytes[i - 1].is_ascii_whitespace() {
                break;
            }
        }

        if b.is_ascii_whitespace() {
            if in_word {
                last_word_end = i;
                in_word = false;
            }
        } else {
            in_word = true;
        }

        i += 1;
    }

    if in_word {
        last_word_end = i;
    }

    &s[..last_word_end]
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::bvc::Book;

    #[test]
    fn lex_simple_characters() {
        let mut lexer = Lexer::new(",:-;");
        let expected_tokens = vec![
            (Token::Comma, ":-;"),
            (Token::Colon, "-;"),
            (Token::Dash, ";"),
            (Token::SemiColon, ""),
        ];
        for expected in expected_tokens {
            let token = lexer.next().unwrap().unwrap();
            let (expected, rest) = expected;
            assert_eq!(token, expected);
            assert_eq!(lexer.rest, rest);
        }
    }

    #[test]
    fn lex_a_single_digit_number() {
        let mut lexer = Lexer::new("1");
        let token = lexer.next().unwrap().unwrap();
        assert_eq!(token, Token::Number(1));
        assert_eq!(lexer.rest, "");
    }

    #[test]
    fn lex_a_multiple_digit_number() {
        let mut lexer = Lexer::new("123");
        assert_eq!(lexer.next().unwrap().unwrap(), Token::Number(123));
        assert_eq!(lexer.rest, "");
    }

    #[test]
    fn lex_separate_digit_number() {
        let mut lexer = Lexer::new("1 123");
        let expected_tokens = vec![(1, " 123"), (123, "")];
        for expected in expected_tokens {
            let token = lexer.next().unwrap().unwrap();
            assert_eq!(token, Token::Number(expected.0));
            assert_eq!(lexer.rest, expected.1);
        }
    }

    #[test]
    fn lex_books() {
        let mut lexer = Lexer::new("Psalms");
        let expected_tokens = vec![(Token::Book(Book::Psalms), "")];
        for expected in expected_tokens {
            let token = lexer.next().unwrap().unwrap();
            let (expected_token, rest) = expected;
            assert_eq!(token, expected_token);
            assert_eq!(lexer.rest, rest);
        }
    }

    #[test]
    fn lex_simple_reference() {
        let mut lexer = Lexer::new("Psalms 1:10");
        let expected_tokens = vec![
            (Token::Book(Book::Psalms), " 1:10"),
            (Token::Number(1), ":10"),
            (Token::Colon, "10"),
            (Token::Number(10), ""),
        ];
        for expected in expected_tokens {
            let token = lexer.next().unwrap().unwrap();
            let (expected_token, rest) = expected;
            assert_eq!(token, expected_token);
            assert_eq!(lexer.rest, rest);
        }
    }

    #[test]
    fn lex_following_verses() {
        let mut lexer = Lexer::new("ff");
        let expected_tokens = vec![(Token::FF, "")];
        for expected in expected_tokens {
            let token = lexer.next().unwrap().unwrap();
            let (expected_token, rest) = expected;
            assert_eq!(token, expected_token);
            assert_eq!(lexer.rest, rest);
        }
    }

    #[test]
    fn lex_books_with_number() {
        let mut lexer = Lexer::new("1 Kings");
        // Now produces a single Book token instead of Number + BookSeries
        let token = lexer.next().unwrap().unwrap();
        assert_eq!(token, Token::Book(Book::FirstKings));
        assert_eq!(lexer.rest, "");
    }

    #[test]
    fn lex_multi_word_books() {
        let mut lexer = Lexer::new("Song of Songs");
        let token = lexer.next().unwrap().unwrap();
        assert_eq!(token, Token::Book(Book::SongOfSongs));
        assert_eq!(lexer.rest, "");
    }

    #[test]
    fn lex_multi_word_book_with_reference() {
        let mut lexer = Lexer::new("Song of Songs 1:2");
        let expected_tokens = vec![
            (Token::Book(Book::SongOfSongs), " 1:2"),
            (Token::Number(1), ":2"),
            (Token::Colon, "2"),
            (Token::Number(2), ""),
        ];
        for expected in expected_tokens {
            let token = lexer.next().unwrap().unwrap();
            let (expected_token, rest) = expected;
            assert_eq!(token, expected_token);
            assert_eq!(lexer.rest, rest);
        }
    }

    #[test]
    fn lex_numbered_book_with_reference() {
        let mut lexer = Lexer::new("1 Kings 2:3");
        let expected_tokens = vec![
            (Token::Book(Book::FirstKings), " 2:3"),
            (Token::Number(2), ":3"),
            (Token::Colon, "3"),
            (Token::Number(3), ""),
        ];
        for expected in expected_tokens {
            let token = lexer.next().unwrap().unwrap();
            let (expected_token, rest) = expected;
            assert_eq!(token, expected_token);
            assert_eq!(lexer.rest, rest);
        }
    }
}
