//! Reference string generators for property testing

use proptest::prelude::*;

use super::book::arb_book;
use super::token::arb_verse_part;

/// Generate a simple "Book Chapter:Verse" reference string
///
/// # Examples
/// "John 3:16", "Genesis 1:1", "Psalms 119:105"
///
/// # Constraints
/// - Chapter: 1-50 (covers most books)
/// - Verse: 1-30 (conservative to avoid out-of-bounds)
pub(crate) fn arb_simple_reference() -> impl Strategy<Value = String> {
    (arb_book(), 1u8..=50u8, 1u8..=30u8)
        .prop_map(|(book, chapter, verse)| format!("{} {}:{}", book, chapter, verse))
}

/// Generate a verse range reference "Book Ch:V1-V2"
///
/// # Examples
/// "Matthew 5:3-12", "Genesis 1:1-5", "John 1:1-14"
pub(crate) fn arb_range_reference() -> impl Strategy<Value = String> {
    (arb_book(), 1u8..=50u8, 1u8..=15u8, 16u8..=30u8)
        .prop_map(|(book, chapter, v1, v2)| format!("{} {}:{}-{}", book, chapter, v1, v2))
}

/// Generate a chapter range reference "Book Ch1-Ch2:V"
///
/// # Examples
/// "Genesis 1-3:5", "Matthew 5-7:28", "John 1-2:11"
pub(crate) fn arb_chapter_range_reference() -> impl Strategy<Value = String> {
    (arb_book(), 1u8..=10u8, 11u8..=20u8, 1u8..=15u8)
        .prop_map(|(book, ch1, ch2, verse)| format!("{} {}-{}:{}", book, ch1, ch2, verse))
}

/// Generate a multi-book reference with semicolon
///
/// # Examples
/// "Matthew 5:3; John 3:16", "Genesis 1:1; Exodus 20:1"
pub(crate) fn arb_multibook_reference() -> impl Strategy<Value = String> {
    (
        arb_book(),
        1u8..=20u8,
        1u8..=15u8,
        arb_book(),
        1u8..=20u8,
        1u8..=15u8,
    )
        .prop_map(|(b1, ch1, v1, b2, ch2, v2)| {
            format!("{} {}:{}; {} {}:{}", b1, ch1, v1, b2, ch2, v2)
        })
}

/// Generate a reference with verse parts
///
/// # Examples
/// "John 1:1a", "Genesis 2:4b"
pub(crate) fn arb_verse_part_reference() -> impl Strategy<Value = String> {
    (arb_book(), 1u8..=10u8, 1u8..=20u8, arb_verse_part()).prop_map(
        |(book, chapter, verse, part)| format!("{} {}:{}{}", book, chapter, verse, part as char),
    )
}

/// Generate a verse part range reference
///
/// # Examples
/// "Genesis 2:4b-5c", "John 1:1a-2b"
pub(crate) fn arb_verse_part_range_reference() -> impl Strategy<Value = String> {
    (
        arb_book(),
        1u8..=10u8,
        1u8..=10u8,
        11u8..=20u8,
        arb_verse_part(),
        arb_verse_part(),
    )
        .prop_map(|(book, chapter, v1, v2, p1, p2)| {
            format!(
                "{} {}:{}{}-{}{}",
                book, chapter, v1, p1 as char, v2, p2 as char
            )
        })
}

/// Generate a complex reference with selections
///
/// # Examples
/// "Genesis 1:1,3,5-7", "Matthew 5:3-7,10-12"
pub(crate) fn arb_selection_reference() -> impl Strategy<Value = String> {
    (arb_book(), 1u8..=10u8)
        .prop_flat_map(|(book, chapter)| {
            (
                Just(book),
                Just(chapter),
                1u8..=5u8,
                7u8..=10u8,
                12u8..=15u8,
            )
        })
        .prop_map(|(book, chapter, v1, v2, v3)| {
            format!("{} {}:{},{}-{}", book, chapter, v1, v2, v3)
        })
}

/// Generate a reference with "ff" (following verses)
///
/// # Examples
/// "Matthew 28:19ff", "John 3:16ff"
pub(crate) fn arb_following_reference() -> impl Strategy<Value = String> {
    (arb_book(), 1u8..=20u8, 1u8..=20u8)
        .prop_map(|(book, chapter, verse)| format!("{} {}:{}ff", book, chapter, verse))
}
