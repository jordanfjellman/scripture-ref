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

    for chapter_num in 1..=Book::Genesis.chapter_count() {
        let chapter_as_passage = ScripturePassageRef::from(Chapter::new(
            Book::Genesis,
            ChapterNumber::new(chapter_num)?,
        )?);
        println!("{chapter_as_passage}");
    }

    Ok(())
}
