use crate::{
    Book, Chapter, Verse,
    bvc::VersePart,
    canon::{Canon, InCanon},
};

// 5 + 7 + 8 + 8 + 4 = 32 bits
// cannon: 5 bits // I don't need to sort by this after using the order!
// book: 7 bits
// chapter: 8 bits
// verse: 8 bits
// part: 4 bits
#[derive(Debug)]
pub(crate) struct ScripturePosition(u32);

#[derive(Debug)]
pub(crate) enum ScriptureEnd {
    /// Points to a real next position (the start of the next verse, chapter, or book)
    NextPosition(ScripturePosition),

    /// A non-real position used to create virtual boundaries - useful for comparisons
    VirtualBoundary(u32),
}

pub(crate) struct ScriptureEndBuilder<'c, C: Canon> {
    canon: &'c C,
    book: Option<Book>,
    chapter: Option<Chapter>,
    verse: Option<Verse>,
    verse_part: Option<VersePart>,
}

impl<'c, C: Canon> ScriptureEndBuilder<'c, C> {
    pub fn new(canon: &'c C) -> Self {
        Self {
            canon,
            book: None,
            chapter: None,
            verse: None,
            verse_part: None,
        }
    }

    pub fn build(self) -> Result<ScriptureEnd, String> {
        // Move from least specific to most specific
        todo!()
    }
}

#[derive(Debug)]
pub(crate) struct ScriptureSpan {
    pub(crate) start: ScripturePosition,
    pub(crate) end: ScriptureEnd,
}

impl ScripturePosition {
    fn new(book_pos: u8, chapter: u8, verse: u8, part: u8) -> Self {
        let position =
            (book_pos as u32) << 24 | (chapter as u32) << 16 | (verse as u32) << 8 | (part as u32);
        Self(position)
    }

    pub(crate) fn as_raw(&self) -> u32 {
        self.0
    }
}

impl ScriptureEnd {
    pub(crate) fn as_raw(&self) -> u32 {
        match self {
            ScriptureEnd::NextPosition(pos) => pos.0,
            ScriptureEnd::VirtualBoundary(pos) => *pos,
        }
    }

    pub(crate) fn into_real_position(self) -> Option<ScripturePosition> {
        match self {
            ScriptureEnd::NextPosition(pos) => Some(pos),
            ScriptureEnd::VirtualBoundary(_) => None,
        }
    }
}

pub(crate) trait SpannedScripture {
    fn start_position(&self) -> Result<ScripturePosition, String>;
    fn end_position(&self) -> Result<ScriptureEnd, String>;

    fn span(&self) -> Result<ScriptureSpan, String> {
        let start = self.start_position()?;
        let end = self.end_position()?;
        Ok(ScriptureSpan { start, end })
    }
}

impl<'c, C: Canon> Iterator for InCanon<'c, Book, C> {
    type Item = Option<Book>;

    fn next(&mut self) -> Option<Self::Item> {
        self.canon
            .book_position(self.inner)
            .map(|cur_pos: u8| self.canon.book_at_position(cur_pos + 1))
    }
}

impl<'c, C: Canon> SpannedScripture for InCanon<'c, Book, C> {
    fn start_position(&self) -> Result<ScripturePosition, String> {
        let pos = self
            .canon
            .book_position(self.inner)
            .ok_or(format!("book {} is not in the canon", self.inner))?;
        Ok(ScripturePosition::new(pos, 0, 0, 0))
    }

    fn end_position(&self) -> Result<ScriptureEnd, String> {
        let cur_pos: u8 = self
            .canon
            .book_position(self.inner)
            .ok_or(format!("book {} is not in the canon", self.inner))?;
        let next_pos = cur_pos + 1;
        let end_pos = self
            .canon
            .book_at_position(next_pos)
            .map_or(ScriptureEnd::VirtualBoundary(u32::MAX), |_| {
                ScriptureEnd::NextPosition(ScripturePosition::new(next_pos, 0, 0, 0))
            });
        Ok(end_pos)
    }
}

impl<'c, C: Canon> SpannedScripture for InCanon<'c, Chapter, C> {
    fn start_position(&self) -> Result<ScripturePosition, String> {
        let book_pos = self
            .canon
            .book_position(self.inner.book)
            .ok_or(format!("book {} is not in the canon", self.inner.book))?;
        let chapter_pos = ScripturePosition::new(book_pos, self.inner.number.get(), 0, 0);
        Ok(chapter_pos)
    }

    fn end_position(&self) -> Result<ScriptureEnd, String> {
        let chapter_count = self.inner.book.chapter_count();
        let book_pos = self
            .canon
            .book_position(self.inner.book)
            .ok_or(format!("book {} is not in the canon", self.inner.book))?;
        if self.inner.number.get() == chapter_count {
            let next_book_pos = book_pos + 1;
            let end_pos = self.canon.book_at_position(next_book_pos).map_or(
                ScriptureEnd::VirtualBoundary(u32::MAX),
                |_| {
                    // TODO: ensure using 0-based chapter number doesn't cause conflicts
                    ScriptureEnd::NextPosition(ScripturePosition::new(next_book_pos, 0, 0, 0))
                },
            );
            Ok(end_pos)
        } else {
            assert!(self.inner.number.get() < chapter_count);
            Ok(ScriptureEnd::NextPosition(ScripturePosition::new(
                book_pos,
                self.inner.number.get() + 1,
                0,
                0,
            )))
        }
    }
}

impl<'c, C: Canon> SpannedScripture for InCanon<'c, Verse, C> {
    fn start_position(&self) -> Result<ScripturePosition, String> {
        let book_pos: u8 = self
            .canon
            .book_position(self.inner.book)
            .ok_or(format!("book {} is not in the canon", self.inner.book))?;
        let verse_pos = ScripturePosition::new(
            book_pos,
            self.inner.chapter.number.get(),
            self.inner.verse.get(),
            0,
        );
        Ok(verse_pos)
    }

    fn end_position(&self) -> Result<ScriptureEnd, String> {
        let chapter_count = self.inner.book.chapter_count();
        // TODO: Should manuscript differences be handled here?
        // From a span perspective, I doubt the exceptions are relevant.
        // This may be a different trait, so that library implementors can specify additional exceptions.
        let max_verse_count = self
            .inner
            .book
            .max_verses_in_chapter(self.inner.chapter.number.get())?;
        let book_pos = self
            .canon
            .book_position(self.inner.book)
            .ok_or(format!("book {} is not in the canon", self.inner.book))?;
        let is_last_verse = self.inner.verse.get() == max_verse_count;
        let is_last_chapter = self.inner.chapter.number.get() == chapter_count;
        todo!()
        // match (is_last_verse, is_last_chapter) {
        //     (true, true) => {
        //     let next_book_pos = book_pos + 1;
        //     let end_pos = self.canon.book_at_position(next_book_pos).map_or(
        //         ScriptureEnd::VirtualBoundary(u32::MAX),
        //         |_| {
        //             // TODO: ensure using 0-based chapter number doesn't cause conflicts
        //             ScriptureEnd::NextPosition(ScripturePosition::new(next_book_pos, 0, 0, 0))
        //         },
        //     );
        //     Ok(end_pos)
        //
        //     },
        //     (true, false) => Ok(ScriptureEnd::NextPosition(ScripturePosition::new(
        //         book_pos,
        //         self.inner.chapter.number.get() + 1,
        //         0,
        //         0,
        //     ))),
        //     (false, true) => Ok(ScriptureEnd::NextPosition(ScripturePosition::new(
        //         book_pos,
        //         self.inner.chapter.number.get() + 1,
        //         0,
        //         0,
        //     ))),
        //     _ => Ok(ScriptureEnd::NextPosition(ScripturePosition::new(
        //         book_pos,
        //         self.inner.verse.get() + 1,
        //         0,
        //         0,
        //     ))),
        // }
    }
}

impl<'c, C: Canon> SpannedScripture for InCanon<'c, VersePart, C> {
    fn start_position(&self) -> Result<ScripturePosition, String> {
        todo!()
    }

    fn end_position(&self) -> Result<ScriptureEnd, String> {
        todo!()
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::{Book, canon::ProtestantCanon};
//
//     #[test]
//     fn test_end_position() {
//         let canon = ProtestantCanon;
//         let book = Book::Genesis;
//         let book_span = InCanon::new(book, &canon);
//         let end_pos = book_span.end_position().unwrap();
//         assert_eq!(end_pos.as_raw(), 0xFF);
//     }
// }
