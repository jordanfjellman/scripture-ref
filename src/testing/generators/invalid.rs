//! Generators for invalid inputs (for error handling tests)

use proptest::prelude::*;

/// Generate invalid book names (random strings that aren't books)
///
/// # Examples
/// "xyz", "notabook", "foobar"
///
/// # Strategy
/// Generates lowercase alphabetic strings that Book::try_from will reject
pub(crate) fn arb_invalid_book_name() -> impl Strategy<Value = String> {
    // Generate 3-10 char strings, filter out valid books
    "[a-z]{3,10}".prop_filter("must not be valid book", |s| {
        crate::bvc::Book::try_from(s.as_str()).is_err()
    })
}

/// Generate invalid punctuation characters
///
/// # Examples
/// '@', '#', '$', '%', '!', etc.
///
/// # Use case
/// Test that lexer properly errors on unsupported characters
pub(crate) fn arb_invalid_punctuation() -> impl Strategy<Value = char> {
    prop::sample::select(vec![
        '@', '#', '$', '%', '^', '&', '*', '(', ')', '!', '?', '<', '>',
    ])
}

/// Generate malformed reference strings
///
/// # Examples
/// "Genesis :1", "John 3:", "Matthew -5", "1:1" (missing book)
pub(crate) fn arb_malformed_reference() -> impl Strategy<Value = String> {
    prop_oneof![
        // Missing verse after colon
        Just("John 3:".to_string()),
        // Missing chapter before colon
        Just("Genesis :1".to_string()),
        // Just numbers (no book)
        Just("1:1".to_string()),
        // Invalid dash usage
        Just("Matthew -5".to_string()),
        // Double punctuation
        Just("Genesis 1::1".to_string()),
        // Empty string
        Just("".to_string()),
    ]
}

/// Generate verse numbers that are too high (> 255)
///
/// # Use case
/// Test that parser rejects or handles integer overflow
///
/// # Edge case boundary
/// These values exceed u8::MAX (255) which is the Token::Number limit
pub(crate) fn arb_overflow_number() -> impl Strategy<Value = u16> {
    256u16..=999u16
}

/// Generate edge-valid numbers (valid for u8 but may exceed actual verse counts)
///
/// # Examples
/// 200, 250, 255 (valid u8, but likely beyond actual verse counts in most chapters)
///
/// # Use case
/// Test boundary conditions - these parse successfully as tokens but may
/// fail semantic validation when checked against actual book/chapter verse counts
pub(crate) fn arb_edge_valid_number() -> impl Strategy<Value = u8> {
    200u8..=255u8
}

/// Generate invalid verse part characters (beyond a-d)
///
/// # Examples
/// 'e', 'f', 'z', etc.
///
/// # Use case
/// Test that only a-d are accepted as verse parts
pub(crate) fn arb_invalid_verse_part() -> impl Strategy<Value = u8> {
    b'e'..=b'z'
}

/// Generate references with edge-valid verse numbers
///
/// # Examples
/// "Genesis 1:250", "John 3:255" (valid token, may fail semantic validation)
pub(crate) fn arb_edge_valid_reference() -> impl Strategy<Value = String> {
    use super::book::arb_book;

    (arb_book(), 1u8..=50u8, arb_edge_valid_number())
        .prop_map(|(book, chapter, verse)| format!("{} {}:{}", book, chapter, verse))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bvc::Book;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn invalid_names_are_invalid(name in arb_invalid_book_name()) {
            prop_assert!(Book::try_from(name.as_str()).is_err());
        }

        #[test]
        fn edge_valid_numbers_are_valid_u8(n in arb_edge_valid_number()) {
            // These should be valid u8 values
            prop_assert!(n >= 200 && n <= 255);
        }

        #[test]
        fn overflow_numbers_exceed_u8(n in arb_overflow_number()) {
            // These should exceed u8::MAX
            prop_assert!(n > u8::MAX as u16);
        }

        #[test]
        fn invalid_verse_parts_beyond_d(part in arb_invalid_verse_part()) {
            // Should be e-z range
            prop_assert!(part >= b'e' && part <= b'z');
        }
    }
}
