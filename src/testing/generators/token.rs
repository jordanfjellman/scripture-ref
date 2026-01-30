//! Token generators for property testing

use crate::parse::lexer::Token;
use proptest::prelude::*;

use super::book::arb_book;

/// Generate valid verse numbers (1-255 for u8 range)
///
/// # Note
/// Upper bound is constrained by Token::Number(u8) type
pub(crate) fn arb_number() -> impl Strategy<Value = u8> {
    1u8..=255u8
}

/// Generate valid verse part labels (a-d)
///
/// # Examples
/// Generates: b'a', b'b', b'c', b'd'
pub(crate) fn arb_verse_part() -> impl Strategy<Value = u8> {
    b'a'..=b'd'
}

/// Generate punctuation characters used in references
///
/// # Examples
/// Generates: ':', ',', '-', ';'
pub(crate) fn arb_punctuation_char() -> impl Strategy<Value = char> {
    prop::sample::select(vec![':', ',', '-', ';'])
}

/// Generate various whitespace patterns
///
/// # Examples
/// Generates: " ", "  ", "\t", " \t ", etc.
pub(crate) fn arb_whitespace() -> impl Strategy<Value = String> {
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
///
/// # Coverage
/// Generates all token variants: Book, Colon, Comma, Dash,
/// SemiColon, FF, Number, VersePart
pub(crate) fn arb_token() -> impl Strategy<Value = Token> {
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

/// Generate a token sequence that requires whitespace separation
///
/// # Examples
/// Generates sequences like: [Book(Genesis), Number(1), Colon, Number(1)]
pub(crate) fn arb_token_sequence_with_spaces() -> impl Strategy<Value = Vec<Token>> {
    prop::collection::vec(arb_token(), 1..10)
}
