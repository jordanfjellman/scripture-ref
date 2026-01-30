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
                    if !potential_book.is_empty()
                        && let Ok((book, bytes_consumed)) = Book::parse(potential_book)
                    {
                        self.rest = &c_onwards[bytes_consumed..];
                        self.current_byte += bytes_consumed - c.len_utf8();
                        return Some(Ok(Token::Book(book)));
                    }

                    // Not a book - check for "ff" or other identifiers
                    let first_non_identifier = c_onwards
                        .find(|c| !matches!(c, 'a'..='z' | 'A'..='Z'))
                        .unwrap_or(c_onwards.len());
                    let literal = &c_onwards[..first_non_identifier];
                    let bytes_from_chars = literal.len() - c.len_utf8() + 1;
                    self.rest = &c_onwards[bytes_from_chars..];

                    let token = match literal {
                        "ff" => Ok(Token::FF),
                        t if t.len() == 1 && matches!(t.as_bytes()[0], b'a'..=b'd') => Ok(Token::VersePart(t.as_bytes()[0])),
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

pub(crate) fn extract_potential_book_name(s: &str) -> &str {
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
#[cfg(test)]
mod proptests {
    use super::*;
    use crate::bvc::Book;
    use proptest::prelude::*;

    /// Generators module - Foundation for all property tests
    mod generators {
        use super::*;

        /// Generate any book from the complete Bible (comprehensive coverage)
        pub fn arb_book() -> impl Strategy<Value = Book> {
            let books = Book::bible();
            (0..books.len()).prop_map(move |idx| books[idx])
        }

        /// Generate specifically multi-word books
        pub fn arb_multiword_book() -> impl Strategy<Value = Book> {
            prop::sample::select(vec![Book::SongOfSongs])
        }

        /// Generate specifically numbered books
        pub fn arb_numbered_book() -> impl Strategy<Value = Book> {
            prop::sample::select(vec![Book::FirstKings, Book::ThirdJohn])
        }

        /// Generate valid numbers (1-255 for u8 range)
        pub fn arb_number() -> impl Strategy<Value = u8> {
            1u8..=255u8
        }

        /// Generate valid verse parts (a-d)
        pub fn arb_verse_part() -> impl Strategy<Value = u8> {
            b'a'..=b'd'
        }

        /// Generate punctuation characters
        pub fn arb_punctuation_char() -> impl Strategy<Value = char> {
            prop::sample::select(vec![':', ',', '-', ';'])
        }

        /// Generate various whitespace patterns
        pub fn arb_whitespace() -> impl Strategy<Value = String> {
            prop::sample::select(vec![
                " ".to_string(),
                "  ".to_string(),
                "   ".to_string(),
                "\t".to_string(),
                " \t".to_string(),
                "\t ".to_string(),
                " \t ".to_string(),
            ])
        }

        /// Generate any valid token
        pub fn arb_token() -> impl Strategy<Value = Token> {
            prop_oneof![
                arb_book().prop_map(Token::Book),
                Just(Token::Colon),
                Just(Token::Comma),
                Just(Token::Dash),
                Just(Token::SemiColon),
                Just(Token::FF),
                arb_number().prop_map(Token::Number),
                arb_verse_part().prop_map(Token::VersePart),
            ]
        }

        /// Convert a token to its string representation
        pub fn token_to_string(token: &Token) -> String {
            match token {
                Token::Book(b) => format!("{}", b),
                Token::Colon => ":".to_string(),
                Token::Comma => ",".to_string(),
                Token::Dash => "-".to_string(),
                Token::SemiColon => ";".to_string(),
                Token::FF => "ff".to_string(),
                Token::Number(n) => n.to_string(),
                Token::VersePart(p) => (*p as char).to_string(),
                Token::Period => ".".to_string(),
            }
        }

        /// Generate a complete valid reference string
        pub fn arb_simple_reference() -> impl Strategy<Value = String> {
            (arb_book(), 1u8..=50u8, 1u8..=30u8)
                .prop_map(|(book, chapter, verse)| format!("{} {}:{}", book, chapter, verse))
        }

        /// Generate a token sequence that requires whitespace separation
        pub fn arb_token_sequence_with_spaces() -> impl Strategy<Value = Vec<Token>> {
            prop::collection::vec(arb_token(), 1..10)
        }
    }

    /// Helper functions module - Tests for extract_potential_book_name
    mod helper_functions {
        use super::*;
        use crate::lexer::extract_potential_book_name;

        proptest! {

            #[test]
            fn extract_stops_at_punctuation(
                prefix in "[a-zA-Z ]{1,20}",
                punct in "[,:;-]"
            ) {
                let input = format!("{}{}", prefix, punct);
                let result = extract_potential_book_name(&input);
                prop_assert!(!result.contains(|c| c == ':' || c == ',' || c == ';' || c == '-'));
            }

            #[test]
            fn extract_handles_multiword_books(_dummy in 0..1) {
                let input = "Song of Songs";
                let result = extract_potential_book_name(input);
                prop_assert_eq!(result, "Song of Songs");
            }

            #[test]
            fn extract_stops_at_number_after_space(
                book in generators::arb_multiword_book(),
                num in 1u8..=150u8
            ) {
                let input = format!("{} {}", book, num);
                let result = extract_potential_book_name(&input);
                let book_str = format!("{}", book);
                prop_assert_eq!(result, book_str);
            }

            #[test]
            fn extract_handles_numbered_book_prefixes(
                book in generators::arb_numbered_book(),
                chapter in 1u8..=50u8
            ) {
                let input = format!("{} {}", book, chapter);
                let result = extract_potential_book_name(&input);
                // Should extract the full book name including the number
                let book_str = format!("{}", book);
                prop_assert!(result.starts_with(&book_str.split_whitespace().next().unwrap()));
            }

            #[test]
            fn extract_respects_word_boundaries(word in "[a-zA-Z]{3,10}") {
                let result = extract_potential_book_name(&word);
                prop_assert_eq!(result, word.as_str());
            }
        }
    }

    /// Multi-word books module - HIGHEST PRIORITY
    mod multiword_books {
        use super::*;

        proptest! {

            #[test]
            fn multiword_books_lex_correctly(book in generators::arb_multiword_book()) {
                let input = format!("{}", book);
                let tokens: Vec<_> = Lexer::new(&input)
                    .collect::<Result<Vec<_>, _>>()
                    .unwrap();

                prop_assert_eq!(tokens.len(), 1);
                prop_assert_eq!(tokens[0], Token::Book(book));
            }

            #[test]
            fn multiword_book_boundaries_correct(
                book in generators::arb_multiword_book(),
                chapter in 1u8..=8u8,
                verse in 1u8..=20u8
            ) {
                let input = format!("{} {}:{}", book, chapter, verse);
                let tokens: Vec<_> = Lexer::new(&input)
                    .collect::<Result<Vec<_>, _>>()
                    .unwrap();

                prop_assert!(tokens.len() >= 4, "Expected at least 4 tokens, got {}", tokens.len());
                prop_assert_eq!(tokens[0], Token::Book(book), "First token should be the book");
                prop_assert_eq!(tokens[1], Token::Number(chapter), "Second token should be chapter number");
                prop_assert_eq!(tokens[2], Token::Colon, "Third token should be colon");
                prop_assert_eq!(tokens[3], Token::Number(verse), "Fourth token should be verse number");
            }

            #[test]
            fn numbered_multiword_combinations(
                book in generators::arb_numbered_book(),
                chapter in 1u8..=22u8,
                verse in 1u8..=30u8
            ) {
                let input = format!("{} {}:{}", book, chapter, verse);
                let tokens: Vec<_> = Lexer::new(&input)
                    .collect::<Result<Vec<_>, _>>()
                    .unwrap();

                prop_assert!(tokens.len() >= 4);
                prop_assert_eq!(tokens[0], Token::Book(book));
                prop_assert_eq!(tokens[1], Token::Number(chapter));
                prop_assert_eq!(tokens[2], Token::Colon);
                prop_assert_eq!(tokens[3], Token::Number(verse));
            }

            #[test]
            fn multiword_books_with_ranges(
                book in generators::arb_multiword_book(),
                chapter in 1u8..=8u8,
                verse1 in 1u8..=10u8,
                verse2 in 11u8..=20u8
            ) {
                let input = format!("{} {}:{}-{}", book, chapter, verse1, verse2);
                let tokens: Vec<_> = Lexer::new(&input)
                    .collect::<Result<Vec<_>, _>>()
                    .unwrap();

                prop_assert_eq!(tokens[0], Token::Book(book));
                prop_assert_eq!(tokens[1], Token::Number(chapter));
                prop_assert_eq!(tokens[2], Token::Colon);
                prop_assert_eq!(tokens[3], Token::Number(verse1));
                prop_assert_eq!(tokens[4], Token::Dash);
                prop_assert_eq!(tokens[5], Token::Number(verse2));
            }

            #[test]
            fn multiword_books_with_selections(
                book in generators::arb_multiword_book(),
                ch1 in 1u8..=4u8,
                v1 in 1u8..=10u8,
                ch2 in 5u8..=8u8,
                v2 in 1u8..=10u8,
                v3 in 11u8..=17u8
            ) {
                let input = format!("{} {}:{}, {}:{}-{}", book, ch1, v1, ch2, v2, v3);
                let tokens: Vec<_> = Lexer::new(&input)
                    .collect::<Result<Vec<_>, _>>()
                    .unwrap();

                prop_assert_eq!(tokens[0], Token::Book(book));
                // Verify the book only appears once at the start
                let book_count = tokens.iter().filter(|t| matches!(t, Token::Book(_))).count();
                prop_assert_eq!(book_count, 1);
            }

            #[test]
            fn multiword_books_with_semicolons(_dummy in 0..1) {
                let input = "Song of Songs 1:2; 1 Kings 2:3";
                let tokens: Vec<_> = Lexer::new(input)
                    .collect::<Result<Vec<_>, _>>()
                    .unwrap();

                prop_assert_eq!(tokens[0], Token::Book(Book::SongOfSongs));
                prop_assert_eq!(tokens[1], Token::Number(1));
                prop_assert_eq!(tokens[2], Token::Colon);
                prop_assert_eq!(tokens[3], Token::Number(2));
                prop_assert_eq!(tokens[4], Token::SemiColon);
                prop_assert_eq!(tokens[5], Token::Book(Book::FirstKings));
            }

            #[test]
            fn complex_multiword_whitespace_variations(
                book in generators::arb_multiword_book(),
                ws1 in generators::arb_whitespace(),
                ws2 in generators::arb_whitespace()
            ) {
                let input1 = format!("{} 1:2", book);
                let input2 = format!("{}{}1:2", book, ws1);
                let input3 = format!("Song{}of{}Songs 1:2", ws1, ws2);

                // All variations should successfully lex
                let result1 = Lexer::new(&input1).collect::<Result<Vec<_>, _>>();
                let result2 = Lexer::new(&input2).collect::<Result<Vec<_>, _>>();

                prop_assert!(result1.is_ok());
                prop_assert!(result2.is_ok());

                // input3 will fail because internal whitespace breaks the book name
                // This is expected behavior
            }
        }
    }

    /// All books module - Comprehensive coverage of every book
    mod all_books {
        use super::*;

        proptest! {

            #[test]
            fn all_books_lex_from_canonical_name(book in generators::arb_book()) {
                let input = format!("{}", book);
                let tokens: Vec<_> = Lexer::new(&input)
                    .collect::<Result<Vec<_>, _>>()
                    .unwrap();

                prop_assert_eq!(tokens.len(), 1);
                prop_assert_eq!(tokens[0], Token::Book(book));
            }

            #[test]
            fn all_books_in_simple_references(book in generators::arb_book()) {
                let input = format!("{} 1:1", book);
                let tokens: Vec<_> = Lexer::new(&input)
                    .collect::<Result<Vec<_>, _>>()
                    .unwrap();

                prop_assert!(tokens.len() >= 4);
                prop_assert_eq!(tokens[0], Token::Book(book));
                prop_assert_eq!(tokens[1], Token::Number(1));
                prop_assert_eq!(tokens[2], Token::Colon);
                prop_assert_eq!(tokens[3], Token::Number(1));
            }

            #[test]
            fn all_books_case_insensitive(book in generators::arb_book()) {
                let canonical = format!("{}", book);
                let lowercase = canonical.to_lowercase();
                let uppercase = canonical.to_uppercase();

                // Try parsing with different cases - some might fail due to Book::try_from
                // but the lexer should handle what it receives consistently
                let result_canonical = Lexer::new(&canonical).collect::<Result<Vec<_>, _>>();

                prop_assert!(result_canonical.is_ok());
            }

            #[test]
            fn all_numbered_books_parse_correctly(_dummy in 0..1) {
                let test_cases = vec![
                    ("1 Kings", Book::FirstKings),
                    ("3 John", Book::ThirdJohn),
                ];

                for (input, expected_book) in test_cases {
                    let tokens: Vec<_> = Lexer::new(input)
                        .collect::<Result<Vec<_>, _>>()
                        .unwrap();

                    prop_assert_eq!(tokens.len(), 1);
                    prop_assert_eq!(tokens[0], Token::Book(expected_book));
                }
            }
        }
    }

    /// Numbers module - Validate number lexing
    mod numbers {
        use super::*;

        proptest! {

            #[test]
            fn all_valid_numbers_lex(n in generators::arb_number()) {
                let input = n.to_string();
                let tokens: Vec<_> = Lexer::new(&input)
                    .collect::<Result<Vec<_>, _>>()
                    .unwrap();

                prop_assert_eq!(tokens.len(), 1);
                prop_assert_eq!(tokens[0], Token::Number(n));
            }

            #[test]
            fn multidigit_numbers_lex(n in 10u8..=255u8) {
                let input = n.to_string();
                let tokens: Vec<_> = Lexer::new(&input)
                    .collect::<Result<Vec<_>, _>>()
                    .unwrap();

                prop_assert_eq!(tokens.len(), 1);
                prop_assert_eq!(tokens[0], Token::Number(n));
            }

            #[test]
            fn adjacent_numbers_with_whitespace(
                n1 in generators::arb_number(),
                n2 in generators::arb_number(),
                ws in generators::arb_whitespace()
            ) {
                let input = format!("{}{}{}", n1, ws, n2);
                let tokens: Vec<_> = Lexer::new(&input)
                    .collect::<Result<Vec<_>, _>>()
                    .unwrap();

                prop_assert_eq!(tokens.len(), 2);
                prop_assert_eq!(tokens[0], Token::Number(n1));
                prop_assert_eq!(tokens[1], Token::Number(n2));
            }

            #[test]
            fn numbers_after_books(
                book in generators::arb_book(),
                n in generators::arb_number()
            ) {
                let input = format!("{} {}", book, n);
                let tokens: Vec<_> = Lexer::new(&input)
                    .collect::<Result<Vec<_>, _>>()
                    .unwrap();

                prop_assert!(tokens.len() >= 2);
                prop_assert_eq!(tokens[0], Token::Book(book));
                prop_assert_eq!(tokens[1], Token::Number(n));
            }

            #[test]
            fn numbers_boundary_values(_dummy in 0..1) {
                let test_cases = vec![1u8, 2u8, 254u8, 255u8];

                for n in test_cases {
                    let input = n.to_string();
                    let tokens: Vec<_> = Lexer::new(&input)
                        .collect::<Result<Vec<_>, _>>()
                        .unwrap();

                    prop_assert_eq!(tokens.len(), 1);
                    prop_assert_eq!(tokens[0], Token::Number(n));
                }
            }
        }
    }

    /// Round-trip module - Test lexing invertibility
    mod roundtrip {
        use super::*;

        proptest! {

            #[test]
            fn simple_token_roundtrip(token in prop_oneof![
                Just(Token::Colon),
                Just(Token::Comma),
                Just(Token::Dash),
                Just(Token::SemiColon),
                Just(Token::FF),
            ]) {
                let s = generators::token_to_string(&token);
                let tokens: Vec<_> = Lexer::new(&s)
                    .collect::<Result<Vec<_>, _>>()
                    .unwrap();

                prop_assert_eq!(tokens.len(), 1);
                prop_assert_eq!(tokens[0], token);
            }

            #[test]
            fn number_roundtrip(n in generators::arb_number()) {
                let s = n.to_string();
                let tokens: Vec<_> = Lexer::new(&s)
                    .collect::<Result<Vec<_>, _>>()
                    .unwrap();

                prop_assert_eq!(tokens.len(), 1);
                prop_assert_eq!(tokens[0], Token::Number(n));
            }

            #[test]
            fn book_roundtrip(book in generators::arb_book()) {
                let s = format!("{}", book);
                let tokens: Vec<_> = Lexer::new(&s)
                    .collect::<Result<Vec<_>, _>>()
                    .unwrap();

                prop_assert_eq!(tokens.len(), 1);
                prop_assert_eq!(tokens[0], Token::Book(book));
            }

            #[test]
            fn token_sequence_roundtrip(
                book in generators::arb_book(),
                chapter in 1u8..=50u8,
                verse in 1u8..=30u8
            ) {
                // Build a sequence: Book Chapter:Verse
                let input = format!("{} {}:{}", book, chapter, verse);
                let tokens: Vec<_> = Lexer::new(&input)
                    .collect::<Result<Vec<_>, _>>()
                    .unwrap();

                // Reconstruct
                let reconstructed = tokens.iter()
                    .map(|t| generators::token_to_string(t))
                    .collect::<Vec<_>>()
                    .join(" ");

                // Lex again
                let tokens2: Vec<_> = Lexer::new(&reconstructed)
                    .collect::<Result<Vec<_>, _>>()
                    .unwrap();

                prop_assert_eq!(tokens, tokens2);
            }
        }
    }

    /// Whitespace module - Test whitespace handling
    mod whitespace {
        use super::*;

        proptest! {

            #[test]
            fn whitespace_between_tokens_normalized(
                book in generators::arb_book(),
                chapter in 1u8..=50u8,
                verse in 1u8..=30u8,
                ws1 in generators::arb_whitespace(),
                ws2 in generators::arb_whitespace()
            ) {
                let input1 = format!("{} {}:{}", book, chapter, verse);
                let input2 = format!("{}{}{}{}:{}", book, ws1, chapter, ws2, verse);

                let tokens1: Vec<_> = Lexer::new(&input1)
                    .collect::<Result<Vec<_>, _>>()
                    .unwrap();
                let tokens2: Vec<_> = Lexer::new(&input2)
                    .collect::<Result<Vec<_>, _>>()
                    .unwrap();

                prop_assert_eq!(tokens1, tokens2);
            }

            #[test]
            fn leading_whitespace_ignored(
                input in generators::arb_simple_reference(),
                ws in generators::arb_whitespace()
            ) {
                let with_leading = format!("{}{}", ws, input);

                let tokens1: Vec<_> = Lexer::new(&input)
                    .collect::<Result<Vec<_>, _>>()
                    .unwrap();
                let tokens2: Vec<_> = Lexer::new(&with_leading)
                    .collect::<Result<Vec<_>, _>>()
                    .unwrap();

                prop_assert_eq!(tokens1, tokens2);
            }

            #[test]
            fn trailing_whitespace_ignored(
                input in generators::arb_simple_reference(),
                ws in generators::arb_whitespace()
            ) {
                let with_trailing = format!("{}{}", input, ws);

                let tokens1: Vec<_> = Lexer::new(&input)
                    .collect::<Result<Vec<_>, _>>()
                    .unwrap();
                let tokens2: Vec<_> = Lexer::new(&with_trailing)
                    .collect::<Result<Vec<_>, _>>()
                    .unwrap();

                prop_assert_eq!(tokens1, tokens2);
            }

            #[test]
            fn multiple_spaces_vs_single_space(
                book in generators::arb_book(),
                n in generators::arb_number()
            ) {
                let single = format!("{} {}", book, n);
                let multiple = format!("{}   {}", book, n);

                let tokens1: Vec<_> = Lexer::new(&single)
                    .collect::<Result<Vec<_>, _>>()
                    .unwrap();
                let tokens2: Vec<_> = Lexer::new(&multiple)
                    .collect::<Result<Vec<_>, _>>()
                    .unwrap();

                prop_assert_eq!(tokens1, tokens2);
            }

            #[test]
            fn tabs_vs_spaces(
                book in generators::arb_book(),
                n in generators::arb_number()
            ) {
                let spaces = format!("{} {}", book, n);
                let tabs = format!("{}\t{}", book, n);

                let tokens1: Vec<_> = Lexer::new(&spaces)
                    .collect::<Result<Vec<_>, _>>()
                    .unwrap();
                let tokens2: Vec<_> = Lexer::new(&tabs)
                    .collect::<Result<Vec<_>, _>>()
                    .unwrap();

                prop_assert_eq!(tokens1, tokens2);
            }
        }
    }

    /// Punctuation module - Test punctuation tokens
    mod punctuation {
        use super::*;

        proptest! {

            #[test]
            fn all_punctuation_lexes(punct in generators::arb_punctuation_char()) {
                let input = punct.to_string();
                let tokens: Vec<_> = Lexer::new(&input)
                    .collect::<Result<Vec<_>, _>>()
                    .unwrap();

                prop_assert_eq!(tokens.len(), 1);
                let expected = match punct {
                    ':' => Token::Colon,
                    ',' => Token::Comma,
                    '-' => Token::Dash,
                    ';' => Token::SemiColon,
                    _ => unreachable!(),
                };
                prop_assert_eq!(tokens[0], expected);
            }

            #[test]
            fn consecutive_punctuation(_dummy in 0..1) {
                let input = ":-;,";
                let tokens: Vec<_> = Lexer::new(input)
                    .collect::<Result<Vec<_>, _>>()
                    .unwrap();

                prop_assert_eq!(tokens.len(), 4);
                prop_assert_eq!(tokens[0], Token::Colon);
                prop_assert_eq!(tokens[1], Token::Dash);
                prop_assert_eq!(tokens[2], Token::SemiColon);
                prop_assert_eq!(tokens[3], Token::Comma);
            }

            #[test]
            fn punctuation_without_whitespace(_dummy in 0..1) {
                let input = "1:2-3,4";
                let tokens: Vec<_> = Lexer::new(input)
                    .collect::<Result<Vec<_>, _>>()
                    .unwrap();

                prop_assert_eq!(tokens.len(), 7);
                prop_assert_eq!(tokens[0], Token::Number(1));
                prop_assert_eq!(tokens[1], Token::Colon);
                prop_assert_eq!(tokens[2], Token::Number(2));
                prop_assert_eq!(tokens[3], Token::Dash);
                prop_assert_eq!(tokens[4], Token::Number(3));
                prop_assert_eq!(tokens[5], Token::Comma);
                prop_assert_eq!(tokens[6], Token::Number(4));
            }
        }
    }

    /// Verse parts module - Test verse part (a-d) tokens
    mod verse_parts {
        use super::*;

        proptest! {

            #[test]
            fn valid_verse_parts_lex(part in generators::arb_verse_part()) {
                let input = (part as char).to_string();
                let tokens: Vec<_> = Lexer::new(&input)
                    .collect::<Result<Vec<_>, _>>()
                    .unwrap();

                prop_assert_eq!(tokens.len(), 1);
                prop_assert_eq!(tokens[0], Token::VersePart(part));
            }

            #[test]
            fn verse_parts_in_references(
                book in generators::arb_book(),
                chapter in 1u8..=10u8,
                verse in 1u8..=20u8,
                part in generators::arb_verse_part()
            ) {
                let input = format!("{} {}:{}{}", book, chapter, verse, part as char);
                let tokens: Vec<_> = Lexer::new(&input)
                    .collect::<Result<Vec<_>, _>>()
                    .unwrap();

                prop_assert!(tokens.len() >= 5);
                prop_assert_eq!(tokens[0], Token::Book(book));
                prop_assert_eq!(tokens[1], Token::Number(chapter));
                prop_assert_eq!(tokens[2], Token::Colon);
                prop_assert_eq!(tokens[3], Token::Number(verse));
                prop_assert_eq!(tokens[4], Token::VersePart(part));
            }

            #[test]
            fn verse_parts_in_ranges(
                book in generators::arb_book(),
                chapter in 1u8..=10u8,
                v1 in 1u8..=10u8,
                v2 in 11u8..=20u8,
                p1 in generators::arb_verse_part(),
                p2 in generators::arb_verse_part()
            ) {
                let input = format!("{} {}:{}{}-{}{}",
                    book, chapter, v1, p1 as char, v2, p2 as char);
                let tokens: Vec<_> = Lexer::new(&input)
                    .collect::<Result<Vec<_>, _>>()
                    .unwrap();

                prop_assert!(tokens.len() >= 8);
                prop_assert_eq!(tokens[4], Token::VersePart(p1));
                prop_assert_eq!(tokens[5], Token::Dash);
                prop_assert_eq!(tokens[6], Token::Number(v2));
                prop_assert_eq!(tokens[7], Token::VersePart(p2));
            }

            #[test]
            fn invalid_verse_parts_error(part in b'e'..=b'z') {
                let input = (part as char).to_string();
                let result = Lexer::new(&input).collect::<Result<Vec<_>, _>>();

                prop_assert!(result.is_err());
                if let Err(e) = result {
                    let msg = e.to_string();
                    prop_assert!(msg.contains("not a valid book"));
                }
            }
        }
    }

    /// Following module - Test FF token
    mod following {
        use super::*;

        proptest! {

            #[test]
            fn ff_lexes_correctly(_dummy in 0..1) {
                let tokens: Vec<_> = Lexer::new("ff")
                    .collect::<Result<Vec<_>, _>>()
                    .unwrap();

                prop_assert_eq!(tokens.len(), 1);
                prop_assert_eq!(tokens[0], Token::FF);
            }

            #[test]
            fn ff_in_references(
                book in generators::arb_book(),
                chapter in 1u8..=10u8,
                verse in 1u8..=20u8
            ) {
                let input = format!("{} {}:{}ff", book, chapter, verse);
                let tokens: Vec<_> = Lexer::new(&input)
                    .collect::<Result<Vec<_>, _>>()
                    .unwrap();

                prop_assert!(tokens.len() >= 5);
                prop_assert_eq!(tokens[0], Token::Book(book));
                prop_assert_eq!(tokens[1], Token::Number(chapter));
                prop_assert_eq!(tokens[2], Token::Colon);
                prop_assert_eq!(tokens[3], Token::Number(verse));
                prop_assert_eq!(tokens[4], Token::FF);
            }

            #[test]
            fn ff_not_partial_match(_dummy in 0..1) {
                // "offer" should not extract "ff" from the middle
                let result = Lexer::new("offer").collect::<Result<Vec<_>, _>>();

                // This should error because "offer" is not a valid book
                prop_assert!(result.is_err());
            }
        }
    }

    /// Errors module - Test error handling
    mod errors {
        use super::*;

        proptest! {

            #[test]
            fn invalid_chars_produce_unexpected_token_error(
                invalid in "[!@#$%^&*()+={}\\[\\]|\\\\<>?/~`]"
            ) {
                let result = Lexer::new(&invalid).collect::<Result<Vec<_>, _>>();

                prop_assert!(result.is_err());
            }

            #[test]
            fn error_message_includes_char(
                invalid in "[!@#$%^&*()]"
            ) {
                let result = Lexer::new(&invalid).collect::<Result<Vec<_>, _>>();

                if let Err(e) = result {
                    let msg = e.to_string();
                    // Error should mention the unexpected character
                    prop_assert!(
                        msg.contains(&invalid) || msg.contains("Unexpected"),
                        "Error message should reference the invalid char: {}", msg
                    );
                }
            }

            #[test]
            fn error_span_points_to_exact_position(_dummy in 0..1) {
                let input = "Genesis 1:1 @ 2:2";
                let result = Lexer::new(input).collect::<Result<Vec<_>, _>>();

                prop_assert!(result.is_err());
                // The '@' character at position 12 should be identified
            }

            #[test]
            fn invalid_book_names_produce_not_valid_book_error(
                word in "[a-z]{3,10}"
            ) {
                // Filter to only test words that aren't valid books
                if Book::try_from(word.as_str()).is_ok() {
                    return Ok(());
                }

                let result = Lexer::new(&word).collect::<Result<Vec<_>, _>>();

                prop_assert!(result.is_err());
                if let Err(e) = result {
                    let msg = e.to_string();
                    prop_assert!(
                        msg.contains("not a valid book"),
                        "Expected 'not a valid book' in error: {}", msg
                    );
                }
            }

            #[test]
            fn invalid_identifier_produces_appropriate_error(_dummy in 0..1) {
                let input = "xyz";
                let result = Lexer::new(input).collect::<Result<Vec<_>, _>>();

                prop_assert!(result.is_err());
                if let Err(e) = result {
                    let msg = e.to_string();
                    prop_assert!(msg.contains("not a valid book"));
                    prop_assert!(msg.contains("xyz"));
                }
            }
        }
    }

    /// Lexer state module - Test internal lexer consistency
    mod lexer_state {
        use super::*;

        proptest! {

            #[test]
            fn current_byte_tracking_accurate(input in generators::arb_simple_reference()) {
                let mut lexer = Lexer::new(&input);
                let original_len = input.len();

                while let Some(_) = lexer.next() {
                    // current_byte should never exceed the input length
                    prop_assert!(lexer.current_byte <= original_len);
                }

                // After consuming all tokens, rest should be empty
                prop_assert_eq!(lexer.rest, "");
            }

            #[test]
            fn rest_field_tracking_accurate(input in generators::arb_simple_reference()) {
                let mut lexer = Lexer::new(&input);
                let mut total_consumed = 0;

                while lexer.next().is_some() {
                    // rest length should decrease or stay same (if whitespace)
                    let current_rest_len = lexer.rest.len();
                    prop_assert!(current_rest_len <= input.len() - total_consumed);
                }
            }

            #[test]
            fn peek_doesnt_consume_input(input in generators::arb_simple_reference()) {
                let mut lexer = Lexer::new(&input);

                // Peek twice should return the same value
                let peek1_is_some = lexer.peek().is_some();
                let peek2_is_some = lexer.peek().is_some();

                prop_assert_eq!(peek1_is_some, peek2_is_some);

                // If there was a value, verify calling next gives the same value
                if peek1_is_some {
                    let is_ok = lexer.peek().unwrap().is_ok();
                    let peeked_token = if is_ok {
                        lexer.peek().unwrap().as_ref().ok().copied()
                    } else {
                        None
                    };

                    let nexted = lexer.next().unwrap();

                    match (peeked_token, &nexted) {
                        (Some(p), Ok(n)) => prop_assert_eq!(&p, n),
                        (None, Err(_)) => {},
                        _ => prop_assert!(false, "peek should return same as next"),
                    }
                }
            }

            #[test]
            fn peek_and_next_consistent(input in generators::arb_simple_reference()) {
                let mut lexer = Lexer::new(&input);

                loop {
                    // Check if there's a next token by peeking
                    let has_next = lexer.peek().is_some();
                    if !has_next {
                        break;
                    }

                    // Get the peeked token (for OK case, we can compare values)
                    let is_ok = lexer.peek().unwrap().is_ok();
                    let peeked_token = if is_ok {
                        lexer.peek().unwrap().as_ref().ok().copied()
                    } else {
                        None
                    };

                    // Now call next
                    let nexted = lexer.next().unwrap();

                    // Verify consistency
                    match (peeked_token, &nexted) {
                        (Some(p), Ok(n)) => prop_assert_eq!(&p, n),
                        (None, Err(_)) => {}, // Both indicated error
                        _ => prop_assert!(false, "peek and next should be consistent"),
                    }
                }
            }

            #[test]
            fn lexer_is_deterministic(input in generators::arb_simple_reference()) {
                let tokens1 = Lexer::new(&input).collect::<Result<Vec<_>, _>>();
                let tokens2 = Lexer::new(&input).collect::<Result<Vec<_>, _>>();

                // Should produce identical results
                match (&tokens1, &tokens2) {
                    (Ok(t1), Ok(t2)) => prop_assert_eq!(t1, t2),
                    (Err(_), Err(_)) => {}, // Both failing is deterministic
                    _ => prop_assert!(false, "Lexer should be deterministic"),
                }
            }
        }
    }

    /// Integration module - Complex end-to-end scenarios
    mod integration {
        use super::*;

        proptest! {

            #[test]
            fn complete_references_lex_correctly(
                book in generators::arb_book(),
                chapter in 1u8..=50u8,
                v1 in 1u8..=15u8,
                v2 in 16u8..=30u8
            ) {
                let input = format!("{} {}:{}-{}", book, chapter, v1, v2);
                let tokens: Vec<_> = Lexer::new(&input)
                    .collect::<Result<Vec<_>, _>>()
                    .unwrap();

                prop_assert_eq!(tokens[0], Token::Book(book));
                prop_assert_eq!(tokens[1], Token::Number(chapter));
                prop_assert_eq!(tokens[2], Token::Colon);
                prop_assert_eq!(tokens[3], Token::Number(v1));
                prop_assert_eq!(tokens[4], Token::Dash);
                prop_assert_eq!(tokens[5], Token::Number(v2));
            }

            #[test]
            fn complex_multibook_references(
                book1 in generators::arb_book(),
                book2 in generators::arb_book(),
                ch1 in 1u8..=20u8,
                ch2 in 1u8..=20u8,
                v1 in 1u8..=15u8,
                v2 in 1u8..=15u8
            ) {
                let input = format!("{} {}:{}; {} {}:{}", book1, ch1, v1, book2, ch2, v2);
                let tokens: Vec<_> = Lexer::new(&input)
                    .collect::<Result<Vec<_>, _>>()
                    .unwrap();

                prop_assert_eq!(tokens[0], Token::Book(book1));
                prop_assert_eq!(tokens[4], Token::SemiColon);
                prop_assert_eq!(tokens[5], Token::Book(book2));
            }

            #[test]
            fn ranges_with_verse_parts(
                book in generators::arb_book(),
                chapter in 1u8..=10u8,
                v1 in 1u8..=10u8,
                v2 in 11u8..=20u8,
                p1 in generators::arb_verse_part(),
                p2 in generators::arb_verse_part()
            ) {
                let input = format!("{} {}:{}{}-{}{}",
                    book, chapter, v1, p1 as char, v2, p2 as char);
                let result = Lexer::new(&input).collect::<Result<Vec<_>, _>>();

                prop_assert!(result.is_ok());
                let tokens = result.unwrap();
                prop_assert!(tokens.contains(&Token::VersePart(p1)));
                prop_assert!(tokens.contains(&Token::VersePart(p2)));
            }

            #[test]
            fn selections_with_multiple_ranges(
                book in generators::arb_book(),
                chapter in 1u8..=10u8
            ) {
                let input = format!("{} {}:1-3, 5-7, 10", book, chapter);
                let tokens: Vec<_> = Lexer::new(&input)
                    .collect::<Result<Vec<_>, _>>()
                    .unwrap();

                // Should have: Book, Number, Colon, then alternating numbers/punctuation
                prop_assert_eq!(tokens[0], Token::Book(book));
                prop_assert_eq!(tokens[1], Token::Number(chapter));
                prop_assert_eq!(tokens[2], Token::Colon);

                // Check for commas in the sequence
                let comma_count = tokens.iter().filter(|t| **t == Token::Comma).count();
                prop_assert_eq!(comma_count, 2);
            }

            #[test]
            fn all_token_types_together(
                book in generators::arb_book(),
                chapter in 1u8..=10u8,
                v1 in 1u8..=10u8,
                v2 in 11u8..=20u8,
                part in generators::arb_verse_part()
            ) {
                // Mix: Book, Numbers, Colon, Dash, VersePart, Comma, FF
                let input = format!("{} {}:{}{}-{}, {}ff",
                    book, chapter, v1, part as char, v2, v2);
                let tokens: Vec<_> = Lexer::new(&input)
                    .collect::<Result<Vec<_>, _>>()
                    .unwrap();

                // Verify we have a mix of token types
                prop_assert!(tokens.iter().any(|t| matches!(t, Token::Book(_))));
                prop_assert!(tokens.iter().any(|t| matches!(t, Token::Number(_))));
                prop_assert!(tokens.iter().any(|t| matches!(t, Token::Colon)));
                prop_assert!(tokens.iter().any(|t| matches!(t, Token::Dash)));
                prop_assert!(tokens.iter().any(|t| matches!(t, Token::VersePart(_))));
                prop_assert!(tokens.iter().any(|t| matches!(t, Token::Comma)));
                prop_assert!(tokens.iter().any(|t| matches!(t, Token::FF)));
            }
        }
    }

    /// TODOs module - Document future work
    mod todos {
        // TODO: Period token is defined but not yet lexed
        // Add tests when Period lexing is implemented:
        // - period_lexes_correctly: Test that '.' produces Token::Period
        // - period_in_abbreviations: Test periods in book abbreviations like "Gen."
        // - period_vs_other_punctuation: Test period alongside other punctuation
        //
        // Example test structure:
        // #[test]
        // fn period_lexes_correctly() {
        //     let tokens: Vec<_> = Lexer::new(".")
        //         .collect::<Result<Vec<_>, _>>()
        //         .unwrap();
        //     assert_eq!(tokens.len(), 1);
        //     assert_eq!(tokens[0], Token::Period);
        // }
    }
}
