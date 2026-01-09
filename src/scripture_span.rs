use crate::{
    bvc::{Book, Chapter, HasBook, Verse, VersePart},
    canon::{Canonical, InCanon},
};

#[derive(Debug)]
pub(crate) struct ScripturePosition(u32);

#[derive(Debug)]
pub(crate) enum ScriptureEnd {
    /// Points to a real position (the start of the next verse, chapter, or book)
    NextPosition(ScripturePosition),

    /// A non-real position used to create virtual boundaries (i.e., the end of a canon)
    VirtualBoundary(u32),
}

impl ScripturePosition {
    pub(crate) fn new(book_pos: u8, chapter: u8, verse: u8, part: u8) -> Self {
        let position =
            (book_pos as u32) << 24 | (chapter as u32) << 16 | (verse as u32) << 8 | (part as u32);
        Self(position)
    }

    pub(crate) fn get(&self) -> u32 {
        self.0
    }
}

impl ScriptureEnd {
    pub(crate) fn get(&self) -> u32 {
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

pub(crate) trait ScriptureSpan {
    type Parent: ScriptureSpan;
    fn start_position(&self) -> Result<ScripturePosition, String>;
    fn end_position(&self) -> Result<ScriptureEnd, String>;
    fn to_parent(&self) -> Self::Parent;
}

impl<'c, C: Canonical> ScriptureSpan for &'c C {
    type Parent = Self;

    fn start_position(&self) -> Result<ScripturePosition, String> {
        Ok(ScripturePosition::new(0, 0, 0, 0))
    }

    fn end_position(&self) -> Result<ScriptureEnd, String> {
        Ok(ScriptureEnd::VirtualBoundary(u32::MAX))
    }

    fn to_parent(&self) -> Self::Parent {
        *self
    }
}

impl<'c, C: Canonical> ScriptureSpan for InCanon<'c, Book, C> {
    type Parent = &'c C;

    fn start_position(&self) -> Result<ScripturePosition, String> {
        Ok(ScripturePosition::new(self.book_pos()?, 0, 0, 0))
    }

    fn end_position(&self) -> Result<ScriptureEnd, String> {
        let next_pos = self.book_pos()? + 1;
        match self.canon.book_at_position(next_pos) {
            Some(_) => Ok(ScriptureEnd::NextPosition(ScripturePosition::new(
                next_pos, 0, 0, 0, // TODO: review the use of 0-based numbers
            ))),
            None => ScriptureSpan::end_position(&self.to_parent()),
        }
    }

    fn to_parent(&self) -> Self::Parent {
        self.canon
    }
}

impl<'c, C: Canonical> ScriptureSpan for InCanon<'c, Chapter, C> {
    type Parent = InCanon<'c, Book, C>;

    fn start_position(&self) -> Result<ScripturePosition, String> {
        Ok(ScripturePosition::new(
            self.book_pos()?,
            self.inner.number.get(),
            0,
            0,
        ))
    }

    fn end_position(&self) -> Result<ScriptureEnd, String> {
        let chapter_count = self.inner.book.chapter_count();
        if self.inner.number.get() < chapter_count {
            Ok(ScriptureEnd::NextPosition(ScripturePosition::new(
                self.book_pos()?,
                self.inner.number.get() + 1,
                0,
                0,
            )))
        } else {
            ScriptureSpan::end_position(&self.to_parent())
        }
    }

    fn to_parent(&self) -> Self::Parent {
        InCanon::new(self.inner.book, self.canon)
    }
}

impl<'c, C: Canonical> ScriptureSpan for InCanon<'c, Verse, C> {
    type Parent = InCanon<'c, Chapter, C>;

    fn start_position(&self) -> Result<ScripturePosition, String> {
        Ok(ScripturePosition::new(
            self.book_pos()?,
            self.inner.chapter.number.get(),
            self.inner.number.get(),
            0,
        ))
    }

    fn end_position(&self) -> Result<ScriptureEnd, String> {
        // TODO: Should manuscript differences be handled here?
        // From a span perspective, I doubt the exceptions are relevant.
        // This may be a different trait, so that library implementors can specify additional exceptions.
        let max_verse = self
            .inner
            .book
            .max_verses_in_chapter(self.inner.chapter.number.get())?;
        if self.inner.number.get() < max_verse {
            Ok(ScriptureEnd::NextPosition(ScripturePosition::new(
                self.book_pos()?,
                self.inner.chapter.number.get(),
                self.inner.number.get() + 1,
                0,
            )))
        } else {
            ScriptureSpan::end_position(&self.to_parent())
        }
    }

    fn to_parent(&self) -> Self::Parent {
        InCanon::new(self.inner.chapter, self.canon)
    }
}

impl<'c, C: Canonical> ScriptureSpan for InCanon<'c, VersePart, C> {
    type Parent = InCanon<'c, Verse, C>;

    fn start_position(&self) -> Result<ScripturePosition, String> {
        Ok(ScripturePosition::new(
            self.book_pos()?,
            self.inner.chapter.number.get(),
            self.inner.verse.number.get(),
            self.inner.part.get(),
        ))
    }

    fn end_position(&self) -> Result<ScriptureEnd, String> {
        // Parts are labeled a=1, b=2, c=3, d=4; max is 'd' (4)
        const MAX_PART: u8 = b'd';
        if self.inner.part.get() < MAX_PART {
            Ok(ScriptureEnd::NextPosition(ScripturePosition::new(
                self.book_pos()?,
                self.inner.chapter.number.get(),
                self.inner.verse.number.get(),
                self.inner.part.get() + 1,
            )))
        } else {
            ScriptureSpan::end_position(&self.to_parent())
        }
    }

    fn to_parent(&self) -> Self::Parent {
        InCanon::new(self.inner.verse, self.canon)
    }
}

impl<'c, T: HasBook, C: Canonical> InCanon<'c, T, C> {
    pub(crate) fn book_pos(&self) -> Result<u8, String> {
        let book = self.inner.book();
        let pos = self
            .canon
            .book_position(book)
            .ok_or(format!("book {} is not in the canon", book))?;
        Ok(pos)
    }
}

impl<'c, C: Canonical> Iterator for InCanon<'c, Book, C> {
    type Item = Option<Book>;

    fn next(&mut self) -> Option<Self::Item> {
        self.canon
            .book_position(self.inner)
            .map(|cur_pos: u8| self.canon.book_at_position(cur_pos + 1))
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
