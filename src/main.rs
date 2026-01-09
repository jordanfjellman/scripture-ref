mod bvc;
mod canon;
mod scripture_ref_builder;
mod scripture_span;

use bvc::{Book, Chapter, ChapterNumber, Spanned, Verse, VerseNumber, VersePartLabel};
use canon::{InCanon, ProtestantCanon};
use scripture_ref_builder::{
    ScripturePassageRef, ScriptureRef, ScriptureSelectionRef, ScriptureVerseRef,
};
use scripture_span::ScriptureSpan;

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

    println!("{}\n\n", ScriptureRef::from(verse_ref));

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
    print!("\n\n");

    let book = Book::Genesis;
    println!(
        "{}\t\t {:#032b} -> {:#032b}",
        book,
        book.start()?.get(),
        book.end()?.get()
    );

    let chapter = Chapter::new(Book::Genesis, ChapterNumber::new(50)?)?;
    println!(
        "{}\t {:#032b} -> {:#032b}",
        chapter,
        chapter.start()?.get(),
        chapter.end()?.get()
    );

    let verse: Verse = Verse::new(Book::Genesis, ChapterNumber::new(3)?, VerseNumber::new(3)?)?;
    println!(
        "{}\t {:#032b} -> {:#032b}",
        verse,
        verse.start()?.get(),
        verse.end()?.get()
    );

    let verse_part = VersePartLabel::new(b'a')?;
    println!("Verse Part: {verse_part}");

    // TODO: I could parse scripture references without a canon, but could not validate ranges
    // across books.
    let canon = ProtestantCanon;
    let book = Book::Revelation;
    let book_span = InCanon::new(book, &canon);

    println!("Book Span");
    println!("=========");
    println!("{:#034b}", book_span.start_position()?.get());
    println!("{:#034b}\n", book_span.end_position()?.get());

    let chapter = Chapter::new(book, ChapterNumber::new(22)?)?;
    let chapter_span = InCanon::new(chapter, &canon);

    println!("Chapter Span");
    println!("============");
    println!("{:#034b}", chapter_span.start_position()?.get());
    println!("{:#034b}\n", chapter_span.end_position()?.get());

    let verse = Verse::new(book, chapter.number, VerseNumber::new(21)?)?;
    let verse_span = InCanon::new(verse, &canon);

    println!("Verse Span");
    println!("============");
    println!("{:#034b}", verse_span.start_position()?.get());
    println!("{:#034b}", verse_span.end_position()?.get());

    Ok(())
}
