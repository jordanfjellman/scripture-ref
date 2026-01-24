use super::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) struct Verse {
    pub(crate) book: Book,
    pub(crate) chapter: Chapter,
    pub(crate) number: VerseNumber,
}

impl Verse {
    pub fn new(book: Book, chapter: ChapterNumber, verse: VerseNumber) -> Result<Self, String> {
        let chapter = Chapter::new(book, chapter)?;
        let max_verse_count = chapter.max_verse_count()?;
        if max_verse_count < verse.get() {
            Err(format!(
                "{chapter} has at most {} verses, not {}",
                max_verse_count,
                verse.get(),
            ))
        } else {
            Ok(Self {
                book,
                chapter,
                number: verse,
            })
        }
    }
}

impl std::fmt::Display for Verse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}:{}", self.book, self.chapter.number, self.number)
    }
}
