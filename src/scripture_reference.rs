#[derive(thiserror::Error, Debug, PartialEq, Eq)]
#[error("Invalid Verse Reference: {0}")]
pub enum ScriptureReferenceError {
    #[error("Verse reference is invalid")]
    InvalidVerse(String),

    #[error("Passage is invalid")]
    InvalidPassage(PassageError),

    #[error("Selection is invalid")]
    InvalidSelection(SelectionError),
}

#[derive(Debug, PartialEq, Eq)]
pub struct VerseReferenceError(String);

impl From<&'static str> for VerseReferenceError {
    fn from(value: &'static str) -> Self {
        VerseReferenceError(value.to_string())
    }
}

impl From<String> for VerseReferenceError {
    fn from(value: String) -> Self {
        VerseReferenceError(value)
    }
}

impl From<VerseReferenceError> for ScriptureReferenceError {
    fn from(value: VerseReferenceError) -> Self {
        ScriptureReferenceError::InvalidVerse(value.0)
    }
}

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
#[error("Invalid Passage: {0}")]
pub enum PassageError {
    #[error("Verse range is invalid")]
    InvalidVerseRange(&'static str),
}

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
#[error("Invalid Selection: {0}")]
pub enum SelectionError {
    #[error("Selection is invalid")]
    InvalidSelection(VerseReferenceError),
}

pub enum Translation {
    ESV,
    KJV,
    Other(String),
}

impl From<&str> for Translation {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "esv" => Translation::ESV,
            "kjv" => Translation::KJV,
            _ => Translation::Other(value.to_string()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Book {
    Genesis = 1,
    Exodus,
    FirstKings,
    SongOfSongs,
    Obadiah,
    Matthew = 40,
}

// TODO: implement Display for Book

impl TryFrom<&str> for Book {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "genesis" => Ok(Book::Genesis),
            "1 kings" => Ok(Book::FirstKings),
            "song of songs" => Ok(Book::SongOfSongs),
            "obadiah" => Ok(Book::Obadiah),
            "matthew" => Ok(Book::Matthew),
            _ => Err(format!("Book is not valid: {}", value)),
        }
    }
}

// impl Book {
//     pub fn chapter_count(&self) -> u8 {
//         match self {
//             Book::Genesis => GENESIS_CHAPTER_COUNT,
//             Book::Exodus => EXODUS_CHAPTER_COUNT,
//             Book::FirstKings => 39,
//             Book::SongOfSongs => 8,
//             Book::Obadiah => 1,
//             Book::Matthew => 28,
//         }
//     }
// }

#[derive(Debug, PartialEq, Eq)]
pub struct VersePhrase(pub u8);

impl TryFrom<u8> for VersePhrase {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'a'..=b'd' => Ok(VersePhrase(value - b'a')),
            _ => Err("Verse phrase is not valid, must be a single letter from a to d"),
        }
    }
}

impl TryFrom<char> for VersePhrase {
    type Error = &'static str;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        (value as u8).try_into()
    }
}

/// ```rust
/// use std::convert::TryFrom;
/// use scripture_ref::VerseNumber;
///
/// assert_eq!(VerseNumber::try_from(1), Ok(VerseNumber(1)));
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct VerseNumber(pub u8);

impl TryFrom<u8> for VerseNumber {
    type Error = &'static str;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if !(1..175).contains(&value) {
            Err("Verse number is invalid; must be positive and not be greater than 175")
        } else {
            Ok(VerseNumber(value))
        }
    }
}

/// ```rust
/// use scripture_ref::{Verse, VerseNumber};
///
/// assert!(Verse::whole(1).is_ok());
/// assert!(Verse::whole(0).is_err());
/// assert!(Verse::whole(177).is_err());
/// assert!(Verse::partial(1, 'a').is_ok());
/// assert!(Verse::partial(1, b'a').is_ok());
/// assert!(Verse::partial(1, 'π').is_err());
/// assert_eq!(Verse::new(1.try_into().ok().unwrap(), None), Verse { number: VerseNumber(1), phrase: None });
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct Verse {
    pub number: VerseNumber,
    pub phrase: Option<VersePhrase>,
}

impl Verse {
    pub fn new(number: VerseNumber, phrase: Option<VersePhrase>) -> Self {
        Self { number, phrase }
    }

    pub fn whole(
        number: impl TryInto<VerseNumber, Error = &'static str>,
    ) -> Result<Self, VerseReferenceError> {
        Ok(Self::new(
            number.try_into().map_err(VerseReferenceError::from)?,
            None,
        ))
    }

    pub fn partial(
        number: impl TryInto<VerseNumber, Error = &'static str>,
        phrase: impl TryInto<VersePhrase, Error = &'static str>,
    ) -> Result<Self, VerseReferenceError> {
        Ok(Self::new(
            number.try_into().map_err(VerseReferenceError::from)?,
            Some(phrase.try_into().map_err(VerseReferenceError::from)?),
        ))
    }
}

#[derive(Debug)]
pub enum VerseRange {
    Single(Verse),              // 1 or 1a
    Range(Verse, Verse),        // 1-3a
    Selection(Vec<VerseRange>), // 1-3,5-7b,10
}

#[derive(Debug)]
pub struct Chapter {
    book: Book,
    number: ChapterNumber,
}

impl Chapter {
    pub fn new(book: Book, number: ChapterNumber) -> Result<Self, String> {
        if number > book.chapters() {
            Err(format!(
                "Chapter {} is out of range for the {:?}",
                chapter, self
            ))
        } else {
            Self { book, number }
        }
    }

    /// ```rust
    /// use scripture_ref::{Book, Chapter};
    ///
    /// assert_eq!(Book::Genesis.chapter_verse_count(1), Ok(31));
    /// assert_eq!(Book::Genesis.chapter_verse_count(50), Ok(26));
    /// ```
    pub fn max_verse_count(&self) -> Result<u8, String> {
        let chapter_num = self.number.0 as usize;
        let index = chapter as usize - 1;
        let verse_count = match self {
            Book::Genesis => GENESIS_CHAPTER_VERSES[index],
            Book::Exodus => EXODUS_CHAPTER_VERSES[index],
            _ => todo!(),
        };
        Ok(verse_count)
    }
}

#[derive(Debug)]
pub struct ChapterNumber(pub u8);

impl TryFrom<u8> for ChapterNumber {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if !(1..150).contains(&value) {
            Err("Chapter out of range; must be positive and not greater than 150")
        } else {
            Ok(ChapterNumber(value))
        }
    }
}

/// ```rust
/// use scripture_ref::{VerseReference};
///
/// assert!(VerseReference::verse_unchecked("Genesis", 1, 1).is_ok());
/// // assert!(VerseReference::verse_unchecked("4 John", 1, 1).is_err());
/// // assert!(VerseReference::verse("esv", "Acts", 8, 37).is_err());
/// // assert!(VerseReference::verse("kjv", "Acts", 8, 37).is_ok());
/// ```
#[derive(Debug)]
pub struct VerseReference {
    pub book: Book,
    pub chapter: ChapterNumber,
    pub verse: Verse,
}

/// ```rust
/// use scripture_ref::{VerseId, VerseNumber, VersePhrase, Chapter, Book};
/// assert_eq!(VerseId::new(Book::Genesis, Chapter(1), VerseNumber(1), Some(VersePhrase(b'a'))).0, 0x01_01_01_61);
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct VerseId(pub u32);

impl VerseId {
    pub fn new(
        book: Book,
        chapter: ChapterNumber,
        verse: VerseNumber,
        phrase: Option<VersePhrase>,
    ) -> Self {
        let id = (book as u32) << 24
            | (chapter.0 as u32) << 16
            | (verse.0 as u32) << 8
            | (phrase.map(|p| p.0 as u32).unwrap_or(0));
        Self(id)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct PassageId(pub u64);

impl VerseReference {
    pub fn partial_verse(
        _translation: impl TryInto<Translation>,
        _book: impl TryInto<Book>,
        _chapter: impl TryInto<ChapterNumber>,
        _verse: impl TryInto<VerseNumber>,
        _phrase: impl TryInto<VersePhrase>,
    ) -> Result<Self, VerseReferenceError> {
        todo!()
    }

    pub fn partial_verse_unchecked(
        book: impl TryInto<Book, Error = String>,
        chapter: impl TryInto<ChapterNumber, Error = &'static str>,
        verse: impl TryInto<VerseNumber, Error = &'static str>,
        phrase: impl TryInto<VersePhrase, Error = &'static str>,
    ) -> Result<Self, VerseReferenceError> {
        Ok(Self {
            book: book.try_into().map_err(VerseReferenceError::from)?,
            chapter: chapter.try_into().map_err(VerseReferenceError::from)?,
            verse: Verse::partial(verse, phrase)?,
        })
    }

    pub fn verse(
        _translation: impl TryInto<Translation>,
        _book: impl TryInto<Book>,
        _chapter: impl TryInto<ChapterNumber>,
        _verse: impl TryInto<VerseNumber>,
    ) -> Result<Self, VerseReferenceError> {
        todo!()
    }

    pub fn verse_unchecked(
        book: impl TryInto<Book, Error = String>,
        chapter: impl TryInto<ChapterNumber, Error = &'static str>,
        verse: impl TryInto<VerseNumber, Error = &'static str>,
    ) -> Result<Self, VerseReferenceError> {
        Ok(Self {
            book: book.try_into().map_err(VerseReferenceError::from)?,
            chapter: chapter.try_into().map_err(VerseReferenceError::from)?,
            verse: Verse::whole(verse)?,
        })
    }
}

#[derive(Debug)]
pub enum ScriptureReference {
    Verse(VerseReference),
    Passage(VerseReference, VerseReference),
    Selection(Vec<VerseReference>),
}
