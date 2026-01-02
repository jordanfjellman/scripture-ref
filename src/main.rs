mod scripture_ref_builder;

use scripture_ref_builder::{
    Book, Chapter, ChapterNumber, ScripturePassageRef, ScriptureRef, ScriptureSelectionRef,
    ScriptureVerseRef, VerseNumber,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let verse_ref = ScriptureVerseRef::builder()
        .book(Book::Genesis)
        .chapter(ChapterNumber::default())
        .verse(VerseNumber::default())
        .build()?;

    let starting_verse = ScriptureVerseRef::builder()
        .book(Book::Genesis)
        .chapter(ChapterNumber::default())
        .verse(VerseNumber::default())
        .build()?;

    let ending_verse = ScriptureVerseRef::builder()
        .book(Book::Genesis)
        .chapter(ChapterNumber::default())
        .verse(VerseNumber::default())
        .build()?;

    let passage_ref = ScripturePassageRef::builder()
        .start_at(starting_verse)
        .end_at(ending_verse)
        .build()?;

    let selection_ref = ScriptureSelectionRef::builder()
        .add_verse(verse_ref)
        .add_passage(passage_ref)
        .build()?;

    println!("{selection_ref}");

    let verse_ref = ScriptureVerseRef::builder()
        .book(Book::Exodus)
        .chapter(ChapterNumber::new(2)?)
        .verse(VerseNumber::new(24)?)
        .build()?;

    println!("{}", ScriptureRef::from(verse_ref));

    for book in Book::old_testament() {
        for chapter_num in 1..=book.chapter_count() {
            let chapter_as_passage =
                ScripturePassageRef::from(Chapter::new(*book, ChapterNumber::new(chapter_num)?)?);
            println!("OT: {chapter_as_passage}");
        }
    }

    for book in Book::new_testament() {
        for chapter_num in 1..=book.chapter_count() {
            let chapter_as_passage =
                ScripturePassageRef::from(Chapter::new(*book, ChapterNumber::new(chapter_num)?)?);
            println!("NT: {chapter_as_passage}");
        }
    }

    for book in Book::bible() {
        for chapter_num in 1..=book.chapter_count() {
            let chapter_as_passage =
                ScripturePassageRef::from(Chapter::new(*book, ChapterNumber::new(chapter_num)?)?);
            println!("Bible: {chapter_as_passage}");
        }
    }

    Ok(())
}
