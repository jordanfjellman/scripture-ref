//! Book generators for property testing

use crate::bvc::Book;
use proptest::prelude::*;

/// Generate any book from the complete Bible (66 books)
///
/// # Examples
/// Generates: Genesis, Exodus, ..., Revelation
///
/// # Shrinking
/// Shrinks toward earlier books in the canon (Genesis first)
pub(crate) fn arb_book() -> impl Strategy<Value = Book> {
    let books = Book::all();
    (0..books.len()).prop_map(move |idx| books[idx])
}

/// Generate only multi-word books (books with spaces in canonical name)
///
/// # Examples
/// Generates: Song of Songs, 1 Samuel, 2 Kings, 1 Chronicles, etc.
///
/// # Implementation
/// Dynamically discovers books by checking for space in canonical_name()
pub(crate) fn arb_multiword_book() -> impl Strategy<Value = Book> {
    let multiword_books: Vec<Book> = Book::all()
        .iter()
        .copied()
        .filter(|b| b.canonical_name().contains(' '))
        .collect();

    // Ensure we found books (sanity check for future refactorings)
    assert!(
        !multiword_books.is_empty(),
        "No multi-word books found - this is unexpected"
    );

    prop::sample::select(multiword_books)
}

/// Generate only numbered books (1 Kings, 2 Samuel, etc.)
///
/// # Examples
/// Generates: 1 Samuel, 2 Samuel, 1 Kings, 2 Kings, 1 Chronicles,
///            2 Chronicles, 1 Corinthians, 2 Corinthians, etc.
///
/// # Implementation
/// Dynamically discovers books whose canonical name starts with a digit
pub(crate) fn arb_numbered_book() -> impl Strategy<Value = Book> {
    let numbered_books: Vec<Book> = Book::all()
        .iter()
        .copied()
        .filter(|b| {
            b.canonical_name()
                .chars()
                .next()
                .map_or(false, |c| c.is_ascii_digit())
        })
        .collect();

    // Ensure we found books (sanity check)
    assert!(
        !numbered_books.is_empty(),
        "No numbered books found - this is unexpected"
    );

    prop::sample::select(numbered_books)
}

/// Generate single-word, non-numbered books
///
/// # Examples
/// Generates: Genesis, Exodus, Psalms, Matthew, John, etc.
///
/// # Use case
/// Testing books without complex name handling
pub(crate) fn arb_simple_book() -> impl Strategy<Value = Book> {
    let simple_books: Vec<Book> = Book::all()
        .iter()
        .copied()
        .filter(|b| {
            let name = b.canonical_name();
            !name.contains(' ')
        })
        .collect();

    assert!(
        !simple_books.is_empty(),
        "No simple books found - this is unexpected"
    );

    prop::sample::select(simple_books)
}

#[cfg(test)]
mod tests {
    use super::*;

    proptest! {
        #[test]
        fn multiword_books_have_spaces(book in arb_multiword_book()) {
            prop_assert!(book.canonical_name().contains(' '));
        }

        #[test]
        fn numbered_books_start_with_digit(book in arb_numbered_book()) {
            let first_char = book.canonical_name().chars().next().unwrap();
            prop_assert!(first_char.is_ascii_digit());
        }

        #[test]
        fn simple_books_no_spaces(book in arb_simple_book()) {
            prop_assert!(!book.canonical_name().contains(' '));
        }

        #[test]
        fn all_books_are_valid(book in arb_book()) {
            // Every generated book should be a valid Bible book
            prop_assert!(Book::all().contains(&book));
        }
    }
}
