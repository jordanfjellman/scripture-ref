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

/// The books of the Bible.
///
/// The IDs of the books are arbitrary, but PERMANENT. Once assigned, a book's ID should never
/// change. This serves as a "primary key" for the book. By convention, the IDs of the first 66
/// books match the default, protestant canon.
#[derive(scripture_ref_derive::Book, Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub(crate) enum Book {
    #[chapters = "50"]
    #[verses = "31,25,24,26,32,22,24,22,29,32,32,20,18,24,21,16,27,33,38,18,34,24,20,67,34,35,46,22,35,43,55,32,20,31,29,43,36,30,23,23,57,38,34,34,28,34,31,22,33,26"]
    Genesis = 1,

    #[chapters = "40"]
    #[verses = "23,35,29,15,33,34,28,23,23,35,35,27,22,22,25,33,22,24,19,16,31,21,15,22,29,22,31,29,20,23,28,20,18,23,16,31,23,17,22,16"]
    Exodus = 2,

    #[chapters = "22"]
    #[verses = "53,46,28,20,32,38,51,66,28,29,43,33,34,31,34,34,24,46,21,43,29,54"]
    #[series = "Kings"] // TODO: should this be a group?
    FirstKings = 11,

    #[chapters = "150"]
    #[verses = "6,11,9,9,13,11,18,10,21,18,7,9,6,7,5,11,15,51,15,10,14,32,6,10,22,11,14,9,11,13,25,11,22,23,28,13,40,23,14,18,14,12,5,27,18,12,10,15,21,23,21,11,7,9,24,14,12,12,18,14,9,13,12,11,14,20,8,36,37,6,24,20,28,23,11,13,21,72,13,20,17,8,19,13,14,17,7,19,53,17,16,16,5,23,11,13,12,9,9,5,8,29,22,35,45,48,43,14,31,7,10,10,9,8,18,19,2,29,176,7,8,9,4,8,5,6,5,6,8,8,3,18,3,3,21,26,9,8,24,14,10,8,12,15,21,10,20,14,9,6"]
    Psalms = 19,

    #[chapters = "8"]
    #[verses = "17,17,13,16,17,15,20,14"]
    SongOfSongs = 22,

    #[chapters = "1"]
    #[verses = "21"]
    Obadiah = 31,

    #[chapters = "28"]
    #[verses = "25,23,17,25,48,34,29,34,38,42,30,50,58,36,39,28,27,35,30,34,46,46,39,51,46,75,66,20"]
    Matthew = 40,

    #[chapters = "21"]
    #[verses = "51,25,36,54,47,71,53,59,41,42,57,50,38,31,27,33,26,40,42,31,25"]
    John = 43,

    #[chapters = "1"]
    #[verses = "15"]
    ThirdJohn = 64,

    #[chapters = "22"]
    #[verses = "20,29,22,11,14,17,17,13,21,11,19,18,18,20,8,21,18,24,21,15,27,21"]
    Revelation = 66,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) struct ChapterNumber(u8);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) struct Chapter {
    pub(crate) book: Book,
    pub(crate) number: ChapterNumber,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) struct VerseNumber(u8);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) struct Verse {
    pub(crate) book: Book,
    pub(crate) chapter: Chapter,
    pub(crate) number: VerseNumber,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) struct VersePartLabel(u8);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) struct VersePart {
    pub(crate) book: Book,
    pub(crate) chapter: Chapter,
    pub(crate) verse: Verse,
    pub(crate) part: VersePartLabel,
}

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

impl Book {
    const OLD_TESTAMENT: [Self; 5] = [
        // TODO: update to 39
        Self::Genesis,
        Self::Exodus,
        Self::FirstKings,
        Self::SongOfSongs,
        Self::Obadiah,
    ];

    const NEW_TESTAMENT: [Self; 1] = [Self::Matthew]; // TODO: update to 27

    const BIBLE: [Self; 6] = {
        // TODO: update to 66
        let mut all = [Book::Genesis; 6];
        let mut i = 0;
        while i < Self::OLD_TESTAMENT.len() {
            all[i] = Self::OLD_TESTAMENT[i];
            i += 1;
        }
        let mut j = 0;
        while j < Self::NEW_TESTAMENT.len() {
            all[i] = Self::NEW_TESTAMENT[j];
            i += 1;
            j += 1;
        }
        all
    };

    pub fn old_testament() -> &'static [Self] {
        &Self::OLD_TESTAMENT
    }

    pub fn new_testament() -> &'static [Self] {
        &Self::NEW_TESTAMENT
    }

    pub fn bible() -> &'static [Self] {
        &Self::BIBLE
    }
}

impl ChapterNumber {
    pub(crate) fn new(value: u8) -> Result<Self, String> {
        if !(1u8..=150u8).contains(&value) {
            Err(format!(
                "chapter {value} is out of range; must be positive and not greater than 150"
            ))
        } else {
            Ok(ChapterNumber(value))
        }
    }

    pub(crate) fn get(&self) -> u8 {
        self.0
    }
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

impl VerseNumber {
    pub(crate) fn new(value: u8) -> Result<Self, String> {
        if !(1u8..=176u8).contains(&value) {
            Err(format!(
                "verse {value} out of range; must be positive and not greater than 176"
            ))
        } else {
            Ok(VerseNumber(value))
        }
    }

    pub(crate) fn get(&self) -> u8 {
        self.0
    }
}

// TODO: should this be more similar to the Verse type?
impl VersePartLabel {
    pub(crate) fn new(value: u8) -> Result<Self, String> {
        if !(b'a'..=b'd').contains(&value) {
            Err(format!(
                "verse phrase {value} is not valid, must be a single letter from a to d"
            ))
        } else {
            Ok(Self(value))
        }
    }

    pub(crate) fn get(&self) -> u8 {
        self.0
    }

    pub(crate) fn max() -> Self {
        Self(b'd') // TODO: share max logic
    }
}

impl std::default::Default for ChapterNumber {
    fn default() -> Self {
        ChapterNumber(1)
    }
}

impl std::fmt::Display for ChapterNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get())
    }
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

impl std::default::Default for VerseNumber {
    fn default() -> Self {
        Self(1)
    }
}

impl std::fmt::Display for VerseNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get())
    }
}

impl std::fmt::Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // This is treated as code so it's english-only, like error messages.
        match self {
            Book::Genesis => write!(f, "Genesis"),
            Book::Exodus => write!(f, "Exodus"),
            Book::FirstKings => write!(f, "1 Kings"),
            Book::Psalms => write!(f, "Psalms"),
            Book::SongOfSongs => write!(f, "Song of Songs"),
            Book::Obadiah => write!(f, "Obadiah"),
            Book::Matthew => write!(f, "Matthew"),
            Book::Revelation => write!(f, "Revelation"),
        }
    }
}

impl std::fmt::Display for Chapter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.book, self.number)
    }
}

impl std::fmt::Display for Verse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}:{}", self.book, self.chapter.number, self.number)
    }
}

impl std::fmt::Display for VersePartLabel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_ascii_uppercase()) // TODO: is this right?
    }
}

impl TryFrom<&str> for Book {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let normalized = value.to_lowercase(); // TODO: Avoid allocation

        #[cfg(feature = "lang-en")]
        match normalized.as_str() {
            "genesis" | "gen" | "gn" => Ok(Book::Genesis),
            "1 kings" => Ok(Book::FirstKings),
            "song of songs" | "song of solomon" => Ok(Book::SongOfSongs),
            "obadiah" => Ok(Book::Obadiah),
            "matthew" => Ok(Book::Matthew),
            _ => Err(format!("not a valid book: {}", value)),
        }

        #[cfg(not(any(feature = "lang-en")))]
        compile_error!("at least one language feature must be enabled (e.g., lang-en)");
    }
}

impl TryFrom<u8> for ChapterNumber {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        ChapterNumber::new(value)
    }
}

impl TryFrom<&str> for ChapterNumber {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let num = value.parse::<u8>().map_err(|e| {
            format!(
                "not a valid chapter number: {}; error: {:?}",
                value,
                e.kind()
            )
        })?;
        ChapterNumber::try_from(num)
    }
}

impl TryFrom<u8> for VerseNumber {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        VerseNumber::new(value)
    }
}

impl TryFrom<&str> for VerseNumber {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let num = value.parse::<u8>().map_err(|e| {
            format!(
                "not a valid chapter number: {}; error: {:?}",
                value,
                e.kind()
            )
        })?;
        VerseNumber::try_from(num)
    }
}

impl<'de> TryFrom<(Option<u8>, &'de BookToken)> for Book {
    type Error = miette::Error;

    fn try_from((book_num, book_token): (Option<u8>, &'de BookToken)) -> Result<Self, Self::Error> {
        let token = match book_num {
            Some(n) => format!("{} {}", n, book_token),
            None => format!("{}", book_token),
        };
        Book::from_str(token.as_str()).wrap_err(FailedParseFromTokenToBook {
            src: token.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use proptest::prelude::*;

    proptest! {
        #[test]
        fn chapter_number_valid_range(n in 1u8..=150u8) {
            let result = ChapterNumber::new(n);
            prop_assert!(result.is_ok());
            prop_assert_eq!(result.unwrap().0, n);
        }

        #[test]
        fn chapter_number_invalid_below(n in 0u8..1u8) {
            prop_assert!(ChapterNumber::new(n).is_err());
        }

        #[test]
        fn chapter_number_invalid_above(n in 151u8..=255u8) {
            prop_assert!(ChapterNumber::new(n).is_err());
        }
    }

    proptest! {
        #[test]
        fn verse_number_valid_range(n in 1u8..=176u8) {
            let result = VerseNumber::new(n);
            prop_assert!(result.is_ok());
            prop_assert_eq!(result.unwrap().0, n);
        }

        #[test]
        fn verse_number_invalid_below(n in 0u8..1u8) {
            prop_assert!(VerseNumber::new(n).is_err());
        }

        #[test]
        fn verse_number_invalid_above(n in 177u8..=255u8) {
            prop_assert!(VerseNumber::new(n).is_err());
        }
    }
}
