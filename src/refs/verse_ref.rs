use crate::bvc::{Book, ChapterNumber, Verse, VerseNumber, VersePartLabel};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct ScriptureVerseRef {
    verse: Verse,
    verse_part: Option<VersePartLabel>,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct ScriptureVerseRefBuilder {
    book: Option<Book>,
    chapter: Option<ChapterNumber>,
    verse: Option<VerseNumber>,
    verse_part: Option<VersePartLabel>,
}

impl ScriptureVerseRef {
    pub(crate) fn new(
        book: Book,
        chapter: ChapterNumber,
        verse: VerseNumber,
        verse_part: Option<VersePartLabel>,
    ) -> Result<Self, String> {
        Ok(Self {
            verse: Verse::new(book, chapter, verse)?,
            verse_part,
        })
    }

    pub(crate) fn to_verse(&self) -> Verse {
        self.verse
    }

    pub(crate) fn to_verse_part(&self) -> Option<VersePartLabel> {
        self.verse_part
    }

    pub(crate) fn builder() -> ScriptureVerseRefBuilder {
        ScriptureVerseRefBuilder::default()
    }
}

impl ScriptureVerseRefBuilder {
    pub(crate) fn new() -> Self {
        Self {
            book: None,
            chapter: None,
            verse: None,
            verse_part: None,
        }
    }

    pub(crate) fn book(mut self, book: Book) -> Self {
        self.book = Some(book);
        self
    }

    pub(crate) fn try_book<T>(self, book: T) -> Result<Self, T::Error>
    where
        T: TryInto<Book>,
    {
        Ok(self.book(book.try_into()?))
    }

    pub(crate) fn chapter(mut self, chapter: ChapterNumber) -> Self {
        self.chapter = Some(chapter);
        self
    }

    pub(crate) fn try_chapter<T>(self, chapter: T) -> Result<Self, T::Error>
    where
        T: TryInto<ChapterNumber>,
    {
        Ok(self.chapter(chapter.try_into()?))
    }

    pub(crate) fn verse(mut self, verse: VerseNumber) -> Self {
        self.verse = Some(verse);
        self
    }

    pub(crate) fn try_verse<T>(self, verse: T) -> Result<Self, T::Error>
    where
        T: TryInto<VerseNumber>,
    {
        Ok(self.verse(verse.try_into()?))
    }

    pub(crate) fn verse_part(mut self, part: VersePartLabel) -> Self {
        self.verse_part = Some(part);
        self
    }

    pub(crate) fn try_verse_part<T>(self, part: T) -> Result<Self, T::Error>
    where
        T: TryInto<VersePartLabel>,
    {
        Ok(self.verse_part(part.try_into()?))
    }

    pub(crate) fn build(&self) -> Result<ScriptureVerseRef, String> {
        let book = self.book.ok_or_else(|| "book is required".to_string())?;
        let chapter = self
            .chapter
            .ok_or_else(|| "chapter is required".to_string())?;
        let verse = self.verse.ok_or_else(|| "verse is required".to_string())?;

        ScriptureVerseRef::new(book, chapter, verse, self.verse_part)
    }
}

impl std::fmt::Display for ScriptureVerseRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(part) = &self.verse_part {
            write!(f, "{}{part}", self.verse)
        } else {
            write!(f, "{}", self.verse)
        }
    }
}

impl std::default::Default for ScriptureVerseRefBuilder {
    fn default() -> Self {
        Self::new()
    }
}
