mod scripture_ref_builder;

use scripture_ref_builder::{
    Book, Chapter, ChapterNumber, ScripturePassageRef, ScriptureRef, ScriptureSelectionRef,
    ScriptureVerseRef, VerseNumber,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let verse_ref = ScriptureVerseRef::builder()
        .try_book("Genesis")?
        .try_chapter(1)?
        .try_verse(1)?
        .build()?;

    let other_verse_ref = ScriptureVerseRef::builder()
        .try_book("Genesis")?
        .try_chapter(1)?
        .try_verse(31)?
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
        .add_verse(other_verse_ref)
        .add_passage(passage_ref)
        .build()?;

    println!("{selection_ref}");

    let verse_ref = ScriptureVerseRef::builder()
        .book(Book::Exodus)
        .chapter(ChapterNumber::new(2)?)
        .verse(VerseNumber::new(24)?)
        .build()?;

    println!("{}", ScriptureRef::from(verse_ref));

    print!("\n\n");
    println!("Old Testament");
    println!("=============");
    for book in Book::old_testament() {
        for chapter_num in 1..=book.chapter_count() {
            let chapter_as_passage =
                ScripturePassageRef::from(Chapter::new(*book, ChapterNumber::new(chapter_num)?)?);
            print!("{chapter_as_passage}, ");
        }
    }

    print!("\n\n");
    println!("New Testament");
    println!("=============");
    for book in Book::new_testament() {
        for chapter_num in 1..=book.chapter_count() {
            let chapter_as_passage =
                ScripturePassageRef::from(Chapter::new(*book, ChapterNumber::new(chapter_num)?)?);
            print!("{chapter_as_passage}, ");
        }
    }

    print!("\n\n");
    println!("Bible");
    println!("=====");
    for book in Book::bible() {
        for chapter_num in 1..=book.chapter_count() {
            let chapter_as_passage =
                ScripturePassageRef::from(Chapter::new(*book, ChapterNumber::new(chapter_num)?)?);
            print!("{chapter_as_passage}, ");
        }
    }

    Ok(())
}
