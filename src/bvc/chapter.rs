use super::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) struct Chapter {
    pub(crate) book: Book,
    pub(crate) number: ChapterNumber,
}

impl Chapter {
    pub fn new(book: Book, number: ChapterNumber) -> Result<Self, String> {
        if book.chapter_count() < number.get() {
            Err(format!(
                "{book} has {} chapters, not {}",
                book.chapter_count(),
                number.get(),
            ))
        } else {
            Ok(Self { book, number })
        }
    }

    pub fn max_verse_count(&self) -> Result<u8, String> {
        let chapter = self.number.get();
        self.book.max_verses_in_chapter(chapter)
    }
}

impl std::fmt::Display for Chapter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.book, self.number)
    }
}
