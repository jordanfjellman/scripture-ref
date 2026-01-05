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
    FirstKings = 11,

    #[chapters = "8"]
    #[verses = "17,17,13,16,17,15,20,14"]
    SongOfSongs = 22,

    #[chapters = "1"]
    #[verses = "21"]
    Obadiah = 31,

    #[chapters = "28"]
    #[verses = "25,23,17,25,48,34,29,34,38,42,30,50,58,36,39,28,27,35,30,34,46,46,39,51,46,75,66,20"]
    Matthew = 40,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) struct ChapterNumber(u8);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) struct Chapter {
    book: Book,
    number: ChapterNumber,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) struct VerseNumber(u8);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) struct Verse {
    book: Book,
    chapter: Chapter,
    verse: VerseNumber,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) struct VersePart(u8);

#[derive(Debug, Clone)]
pub(crate) enum SelectionPart {
    Verse(ScriptureVerseRef),
    Passage(ScripturePassageRef),
}

// TODO: import instead
#[derive(Debug)]
pub(crate) enum ScriptureRef {
    Verse(ScriptureVerseRef),
    Passage(ScripturePassageRef),
    Selection(ScriptureSelectionRef),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) struct ScriptureVerseRef {
    verse: Verse,
    verse_part: Option<VersePart>,
}

#[derive(Debug, Clone)]
pub(crate) struct ScripturePassageRef {
    start: ScriptureVerseRef,
    end: ScriptureVerseRef,
}

#[derive(Debug, Clone)]
pub(crate) struct ScriptureSelectionRef(Vec<SelectionPart>);

#[derive(Debug)]
pub(crate) struct ScriptureVerseRefBuilder {
    book: Option<Book>,
    chapter: Option<ChapterNumber>,
    verse: Option<VerseNumber>,
    verse_part: Option<VersePart>,
}

#[derive(Debug)]
pub(crate) struct ScripturePassageRefBuilder {
    start: Option<ScriptureVerseRef>,
    end: Option<ScriptureVerseRef>,
}

#[derive(Debug)]
pub(crate) struct ScriptureSelectionRefBuilder {
    selection: Vec<SelectionPart>,
}

pub(crate) struct ScripturePosition(u32);

impl ScripturePosition {
    pub(crate) fn new(
        book: Book,
        chapter: ChapterNumber,
        verse: VerseNumber,
        phrase: Option<VersePart>,
    ) -> Self {
        let position = (book as u32) << 24
            | (chapter.get() as u32) << 16
            | (verse.get() as u32) << 8
            | (phrase.map(|p| p.get() as u32).unwrap_or(0));
        Self(position)
    }

    pub(crate) fn get(&self) -> u32 {
        self.0
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
            Some(VersePart::max()),
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
            Some(VersePart::max()),
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
            self.verse,
            None,
        ))
    }

    fn end(&self) -> Result<Self::Position, Self::Error> {
        Ok(ScripturePosition::new(
            self.book,
            self.chapter.number,
            self.verse,
            Some(VersePart::max()),
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
impl VersePart {
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

impl ScriptureVerseRef {
    pub fn new(
        book: Book,
        chapter: ChapterNumber,
        verse: VerseNumber,
        verse_part: Option<VersePart>,
    ) -> Result<Self, String> {
        Ok(Self {
            verse: Verse::new(book, chapter, verse)?,
            verse_part,
        })
    }

    pub fn builder() -> ScriptureVerseRefBuilder {
        ScriptureVerseRefBuilder::new()
    }
}

impl ScriptureVerseRefBuilder {
    pub fn new() -> Self {
        Self {
            book: None,
            chapter: None,
            verse: None,
            verse_part: None,
        }
    }

    pub fn book(mut self, book: Book) -> Self {
        self.book = Some(book);
        self
    }

    pub fn try_book<T>(self, book: T) -> Result<Self, T::Error>
    where
        T: TryInto<Book>,
    {
        Ok(self.book(book.try_into()?))
    }

    pub fn chapter(mut self, chapter: ChapterNumber) -> Self {
        self.chapter = Some(chapter);
        self
    }

    pub fn try_chapter<T>(self, chapter: T) -> Result<Self, T::Error>
    where
        T: TryInto<ChapterNumber>,
    {
        Ok(self.chapter(chapter.try_into()?))
    }

    pub fn verse(mut self, verse: VerseNumber) -> Self {
        self.verse = Some(verse);
        self
    }

    pub fn try_verse<T>(self, verse: T) -> Result<Self, T::Error>
    where
        T: TryInto<VerseNumber>,
    {
        Ok(self.verse(verse.try_into()?))
    }

    pub fn build(&self) -> Result<ScriptureVerseRef, String> {
        let book = self.book.ok_or_else(|| "book is required".to_string())?;
        let chapter = self
            .chapter
            .ok_or_else(|| "chapter is required".to_string())?;
        let verse = self.verse.ok_or_else(|| "verse is required".to_string())?;

        ScriptureVerseRef::new(book, chapter, verse, self.verse_part)
    }
}

impl ScripturePassageRef {
    pub fn new(start: ScriptureVerseRef, end: ScriptureVerseRef) -> Result<Self, String> {
        Ok(Self { start, end })
    }

    pub fn builder() -> ScripturePassageRefBuilder {
        ScripturePassageRefBuilder::new()
    }
}

impl ScripturePassageRefBuilder {
    pub fn new() -> Self {
        Self {
            start: None,
            end: None,
        }
    }

    pub fn start_at(mut self, verse: ScriptureVerseRef) -> Self {
        self.start = Some(verse);
        self
    }

    pub fn end_at(mut self, verse: ScriptureVerseRef) -> Self {
        self.end = Some(verse);
        self
    }

    pub fn build(&self) -> Result<ScripturePassageRef, String> {
        // TODO: should I handle misorderd or unordered verse refs?
        let start = self
            .start
            .ok_or_else(|| "starting verse ref is required".to_string())?;
        let end = self
            .end
            .ok_or_else(|| "ending verse ref is required".to_string())?;
        ScripturePassageRef::new(start, end)
    }
}

impl ScriptureSelectionRef {
    pub fn new(selection: Vec<SelectionPart>) -> Result<Self, String> {
        Ok(Self(selection))
    }

    pub fn parts(self) -> Vec<SelectionPart> {
        self.0
    }

    pub fn builder() -> ScriptureSelectionRefBuilder {
        ScriptureSelectionRefBuilder::new()
    }
}

impl ScriptureSelectionRefBuilder {
    pub fn new() -> Self {
        Self {
            selection: Vec::new(),
        }
    }

    pub fn add_verse(mut self, verse: ScriptureVerseRef) -> Self {
        self.selection.push(SelectionPart::Verse(verse));
        self
    }

    pub fn add_passage(mut self, passage: ScripturePassageRef) -> Self {
        self.selection.push(SelectionPart::Passage(passage));
        self
    }

    pub fn build(self) -> Result<ScriptureSelectionRef, String> {
        // TODO: should I fail if there are no selections?
        ScriptureSelectionRef::new(self.selection)
    }
}

impl Chapter {
    pub fn new(book: Book, number: ChapterNumber) -> Result<Self, String> {
        Ok(Self { book, number })
    }

    pub fn max_verse_count(&self) -> Result<u8, String> {
        let chapter = self.number.get();
        self.book.max_verses_in_chapter(chapter)
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
        Ok(Self {
            book,
            chapter,
            verse,
        })
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
            Book::SongOfSongs => write!(f, "Song of Songs"),
            Book::Obadiah => write!(f, "Obadiah"),
            Book::Matthew => write!(f, "Matthew"),
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
        write!(f, "{} {}:{}", self.book, self.chapter.number, self.verse)
    }
}

impl std::fmt::Display for VersePart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_ascii_uppercase()) // TODO: is this right?
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

impl std::fmt::Display for ScripturePassageRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: optimize, probably done using ref ids
        if self.start.verse == self.end.verse {
            write!(f, "{}", self.start)
        } else if self.start.verse.chapter == self.end.verse.chapter {
            write!(
                f,
                "{} {}",
                self.start.verse.book, self.start.verse.chapter.number
            )
        } else if self.start.verse.book == self.end.verse.book {
            write!(
                f,
                "{} {}:{}-{}",
                self.start.verse.book,
                self.start.verse.chapter,
                self.start.verse.verse,
                self.end.verse.verse
            )
        } else {
            write!(f, "{}-{}", self.start, self.end)
        }
    }
}

impl std::fmt::Display for SelectionPart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SelectionPart::Verse(v) => write!(f, "{}", v),
            SelectionPart::Passage(p) => write!(f, "{}", p),
        }
    }
}

impl std::fmt::Display for ScriptureSelectionRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut parts = self.clone().parts();
        // TODO: should dedup be done on construction instead? (feels like misplaced logic)
        parts.dedup_by(|a, b| match (a, b) {
            (SelectionPart::Verse(a), SelectionPart::Verse(b)) => a == b,
            _ => false, // TODO: handle passages
        });
        write!(
            f,
            "{}",
            parts
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<String>>()
                .join("; ")
        )
    }
}

impl std::fmt::Display for ScriptureRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScriptureRef::Verse(v) => write!(f, "{}", v),
            ScriptureRef::Passage(p) => write!(f, "{}", p),
            ScriptureRef::Selection(s) => write!(f, "{}", s),
        }
    }
}

impl std::fmt::Display for ScripturePosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get())
    }
}

impl From<ScriptureVerseRef> for ScriptureRef {
    fn from(value: ScriptureVerseRef) -> Self {
        Self::Verse(value)
    }
}

impl From<ScripturePassageRef> for ScriptureRef {
    fn from(value: ScripturePassageRef) -> Self {
        Self::Passage(value)
    }
}

impl From<Chapter> for ScripturePassageRef {
    fn from(chapter: Chapter) -> Self {
        Self::builder()
            .start_at(
                ScriptureVerseRef::builder()
                    .book(chapter.book)
                    .chapter(chapter.number)
                    .verse(VerseNumber::default())
                    .build()
                    .unwrap(),
            )
            .end_at(
                ScriptureVerseRef::builder()
                    .book(chapter.book)
                    .chapter(chapter.number)
                    .verse(chapter.max_verse_count().unwrap().try_into().unwrap())
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap()
    }
}

impl From<ScriptureSelectionRef> for ScriptureRef {
    fn from(value: ScriptureSelectionRef) -> Self {
        Self::Selection(value)
    }
}

// #[derive(Debug)]
// pub(crate) struct IntoBookError(String);
//
// impl From<String> for IntoBookError {
//     fn from(value: String) -> Self {
//         Self(value)
//     }
// }
//
// impl std::fmt::Display for IntoBookError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.0)
//     }
// }
//
// impl std::error::Error for IntoBookError {}

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

impl TryFrom<u8> for VerseNumber {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        VerseNumber::new(value)
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
