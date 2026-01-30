mod book;
mod chapter;
mod chapter_number;
mod verse;
mod verse_number;
mod verse_part;
mod verse_part_label;

pub(crate) use book::Book;
pub(crate) use chapter::Chapter;
pub(crate) use chapter_number::ChapterNumber;
pub(crate) use verse::Verse;
pub(crate) use verse_number::VerseNumber;
pub(crate) use verse_part::VersePart;
pub(crate) use verse_part_label::VersePartLabel;

// Genesis 1:1 is the scripture reference for the following verse.
// |-----------------------------------------------------|
// In the beginning God created the heavens and the earth.
//
// The position of the start and the end of the scripture reference is defined by a u32 value.
// 0b0001_0001_0001_0000 -> 0b0001_0001_0002_0000 (exclusive comparision for the end)
//
// But this is difficult to validate, because at the end of chapters or books, the "next verse u8"
// may not exist.
//
// Genesis 1:1b is the Scripture Reference sub-part for the following verse.
//                 |------------------------------------|
// In the beginning God created the heavens and the earth.
//

// TODO: should there be a concept of "ordered" vs "unordered" books?
// TODO: how should sorting be handled or books be validated across canons?
//

#[derive(Debug)]
pub(crate) struct ScripturePosition(u32);

impl ScripturePosition {
    pub(crate) fn new(
        book: Book,
        chapter: ChapterNumber,
        verse: VerseNumber,
        part: Option<VersePartLabel>,
    ) -> Self {
        let position = (book as u32) << 24
            | (chapter.get() as u32) << 16
            | (verse.get() as u32) << 8
            | (part.map(|p| p.get() as u32).unwrap_or(0));
        Self(position)
    }

    pub(crate) fn get(&self) -> u32 {
        self.0
    }
}

pub(crate) trait HasBook {
    fn book(&self) -> Book;
}

impl HasBook for Book {
    fn book(&self) -> Book {
        *self
    }
}

impl HasBook for Chapter {
    fn book(&self) -> Book {
        self.book
    }
}

impl HasBook for Verse {
    fn book(&self) -> Book {
        self.book
    }
}

impl HasBook for VersePart {
    fn book(&self) -> Book {
        self.book
    }
}

pub(crate) trait Spanned {
    type Position;
    type Error;
    fn start(&self) -> Result<Self::Position, Self::Error>;
    fn end(&self) -> Result<Self::Position, Self::Error>;
}

impl Spanned for Book {
    type Position = ScripturePosition;
    type Error = String;
    fn start(&self) -> Result<Self::Position, Self::Error> {
        Ok(ScripturePosition::new(
            *self,
            ChapterNumber::default(),
            VerseNumber::default(),
            None,
        ))
    }

    fn end(&self) -> Result<Self::Position, Self::Error> {
        let last_chapter: ChapterNumber = self.chapter_count().try_into()?;
        let last_verse = self.max_verses_in_chapter(last_chapter.get())?.try_into()?;
        Ok(ScripturePosition::new(
            *self,
            last_chapter,
            last_verse,
            Some(VersePartLabel::max()),
        ))
    }
}

impl Spanned for Chapter {
    type Position = ScripturePosition;

    type Error = String;

    fn start(&self) -> Result<Self::Position, Self::Error> {
        Ok(ScripturePosition::new(
            self.book,
            self.number,
            VerseNumber::default(),
            None,
        ))
    }

    fn end(&self) -> Result<Self::Position, Self::Error> {
        let last_verse = self.max_verse_count()?.try_into()?;
        Ok(ScripturePosition::new(
            self.book,
            self.number,
            last_verse,
            Some(VersePartLabel::max()),
        ))
    }
}

impl Spanned for Verse {
    type Position = ScripturePosition;

    type Error = String;

    fn start(&self) -> Result<Self::Position, Self::Error> {
        Ok(ScripturePosition::new(
            self.book,
            self.chapter.number,
            self.number,
            None,
        ))
    }

    fn end(&self) -> Result<Self::Position, Self::Error> {
        Ok(ScripturePosition::new(
            self.book,
            self.chapter.number,
            self.number,
            Some(VersePartLabel::max()),
        ))
    }
}

impl std::fmt::Display for ScripturePosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get())
    }
}
