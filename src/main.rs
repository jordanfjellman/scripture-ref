mod scripture_ref_builder;

use scripture_ref_builder::{
    Book, ChapterNumber, ScripturePassageRef, ScriptureSelectionRef, ScriptureVerseRef, VerseNumber,
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
        .chapter(ChapterNumber::new(2))
        .verse(VerseNumber::new(24))
        .build()?;

    println!("{verse_ref}");

    Ok(())
}
