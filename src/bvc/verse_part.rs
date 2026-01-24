use super::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) struct VersePart {
    pub(crate) book: Book,
    pub(crate) chapter: Chapter,
    pub(crate) verse: Verse,
    pub(crate) part: VersePartLabel,
}
