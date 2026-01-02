// TODO: import instead
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub(crate) enum Book {
    Genesis = 1,
    Exodus,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct ChapterNumber(u8);

#[derive(Debug, Clone, Copy)]
pub(crate) struct Chapter {
    book: Book,
    number: ChapterNumber,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct VerseNumber(u8);

#[derive(Debug, Clone, Copy)]
pub(crate) struct Verse {
    book: Book,
    chapter: Chapter,
    verse: VerseNumber,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct VersePart(u8);

#[derive(Debug, Clone)]
pub(crate) enum SelectionPart {
    Verse(ScriptureVerseRef),
    Passage(ScripturePassageRef),
}

// TODO: import instead
#[derive(Debug)]
enum ScriptureRef {
    Verse(ScriptureVerseRef),
    Passage(ScripturePassageRef),
    Selection(ScriptureSelectionRef),
}

#[derive(Debug, Clone, Copy)]
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

impl ChapterNumber {
    pub(crate) fn new(value: u8) -> Self {
        Self(value)
    }
}

impl VerseNumber {
    pub(crate) fn new(value: u8) -> Self {
        Self(value)
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

    pub fn chapter(mut self, chapter: ChapterNumber) -> Self {
        self.chapter = Some(chapter);
        self
    }

    pub fn verse(mut self, verse: VerseNumber) -> Self {
        self.verse = Some(verse);
        self
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
        // TODO: can I handle unordered verse refs?
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
}

impl std::default::Default for ChapterNumber {
    fn default() -> Self {
        ChapterNumber(1)
    }
}

impl std::fmt::Display for ChapterNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
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
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Book::Genesis => write!(f, "Genesis"),
            Book::Exodus => write!(f, "Exodus"),
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
        // TODO: optimize, handle removing book and chapter duplicates (probably done using ref ids)
        write!(f, "{}-{}", self.start, self.end)
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
        write!(
            f,
            "{}",
            self.clone() // TODO: find a way to avoid cloning
                .parts()
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<String>>()
                .join("; ")
        )
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_scripture_verse_ref_builder() {
        unimplemented!()
    }
}
