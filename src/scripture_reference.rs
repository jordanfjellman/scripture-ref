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

#[derive(Debug)]
#[repr(u8)]
pub enum Book {
    Genesis,
    Exodus,
    FirstKings,
    SongOfSongs,
    Obadiah,
    Matthew,
}

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
pub struct VerseNumber(pub u16);

impl TryFrom<u16> for VerseNumber {
    type Error = &'static str;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value < 1 || value > 175 {
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
pub struct Chapter(pub u16);

impl TryFrom<u16> for Chapter {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value < 1 || value > 150 {
            Err("Chapter out of range; must be positive and not greater than 150")
        } else {
            Ok(Chapter(value))
        }
    }
}

/// ```rust
/// use scripture_ref::{VerseReference};
///
/// assert!(VerseReference::verse_unchecked("Genesis", 1, 1).is_ok());
/// assert!(VerseReference::verse_unchecked("4 John", 1, 1).is_err());
/// assert!(VerseReference::verse("esv", "Acts", 8, 37).is_err());
/// assert!(VerseReference::verse("kjv", "Acts", 8, 37).is_ok());
/// ```
#[derive(Debug)]
pub struct VerseReference {
    pub book: Book,
    pub chapter: Chapter,
    pub verse: Verse,
}

impl VerseReference {
    pub fn partial_verse(
        _translation: impl TryInto<Translation>,
        _book: impl TryInto<Book>,
        _chapter: impl TryInto<Chapter>,
        _verse: impl TryInto<VerseNumber>,
        _phrase: impl TryInto<VersePhrase>,
    ) -> Result<Self, VerseReferenceError> {
        todo!()
    }

    pub fn partial_verse_unchecked(
        book: impl TryInto<Book, Error = String>,
        chapter: impl TryInto<Chapter, Error = &'static str>,
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
        _chapter: impl TryInto<Chapter>,
        _verse: impl TryInto<VerseNumber>,
    ) -> Result<Self, VerseReferenceError> {
        todo!()
    }

    pub fn verse_unchecked(
        book: impl TryInto<Book, Error = String>,
        chapter: impl TryInto<Chapter, Error = &'static str>,
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
