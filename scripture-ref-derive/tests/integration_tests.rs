#[derive(scripture_ref_derive::Book, Debug)]
enum Book {
    #[chapters = "3"]
    #[verses = "31, 25, 24"]
    Alpha = 1,

    #[chapters = "2"]
    #[verses = "22, 17"]
    Beta = 2,
}

#[test]
fn test_chapter_count() {
    assert_eq!(Book::Alpha.chapter_count(), 3);
    assert_eq!(Book::Beta.chapter_count(), 2);
}

#[test]
fn test_max_verse_count_by_chapter() {
    assert_eq!(Book::Alpha.max_verse_count_by_chapter(), &[31, 25, 24]);
    assert_eq!(Book::Beta.max_verse_count_by_chapter(), &[22, 17]);
}

#[test]
fn test_max_verses_in_chapter() {
    // Valid chapters (1-indexed)
    assert_eq!(Book::Alpha.max_verses_in_chapter(1).unwrap(), 31);
    assert_eq!(Book::Alpha.max_verses_in_chapter(3).unwrap(), 24);
    assert_eq!(Book::Beta.max_verses_in_chapter(2).unwrap(), 17);

    // Invalid chapters
    assert!(Book::Alpha.max_verses_in_chapter(0).is_err()); // 0 is invalid
    assert!(Book::Alpha.max_verses_in_chapter(4).is_err()); // out of bounds
    assert!(Book::Beta.max_verses_in_chapter(3).is_err()); // out of bounds
}

#[test]
fn test_verse_count() {
    assert_eq!(Book::Alpha.verse_count(), 31 + 25 + 24);
    assert_eq!(Book::Beta.verse_count(), 22 + 17);
}
