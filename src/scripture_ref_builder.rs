use crate::bvc::{
    Book, Chapter, ChapterNumber, ScripturePosition, Verse, VerseNumber, VersePartLabel,
};

#[derive(Debug, Clone)]
pub(crate) enum SelectionPart {
    Verse(ScriptureVerseRef),
    Passage(ScripturePassageRef),
}

// TODO: import instead
#[derive(Debug)]
pub enum ScriptureRef {
    Verse(ScriptureVerseRef),
    Passage(ScripturePassageRef),
    Selection(ScriptureSelectionRef),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) struct ScriptureVerseRef {
    verse: Verse,
    verse_part: Option<VersePartLabel>,
}

#[derive(Debug, Clone)]
pub(crate) struct ScripturePassageRef {
    start: ScriptureVerseRef,
    end: ScriptureVerseRef,
}

#[derive(Debug, Clone)]
pub(crate) struct ScriptureSelectionRef(Vec<SelectionPart>);

#[derive(Debug, Clone, Copy)]
pub(crate) struct ScriptureVerseRefBuilder {
    book: Option<Book>,
    chapter: Option<ChapterNumber>,
    verse: Option<VerseNumber>,
    verse_part: Option<VersePartLabel>,
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

impl ScriptureVerseRef {
    pub fn new(
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

    pub fn builder() -> ScriptureVerseRefBuilder {
        ScriptureVerseRefBuilder::default()
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

    pub fn verse_part(mut self, part: VersePartLabel) -> Self {
        self.verse_part = Some(part);
        self
    }

    pub fn try_verse_part<T>(self, part: T) -> Result<Self, T::Error>
    where
        T: TryInto<VersePartLabel>,
    {
        Ok(self.verse_part(part.try_into()?))
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

impl std::default::Default for ScriptureVerseRefBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl std::default::Default for ScripturePassageRefBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ScripturePassageRef {
    pub fn new(start: ScriptureVerseRef, end: ScriptureVerseRef) -> Result<Self, String> {
        Ok(Self { start, end })
    }

    pub fn builder() -> ScripturePassageRefBuilder {
        ScripturePassageRefBuilder::default()
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

    pub fn add_scripture_ref(self, scripture_ref: ScriptureRef) -> Self {
        match scripture_ref {
            ScriptureRef::Verse(v) => self.add_verse(v),
            ScriptureRef::Passage(p) => self.add_passage(p),
            ScriptureRef::Selection(s) => self.add_selection(s),
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

    pub fn add_selection(mut self, selection: ScriptureSelectionRef) -> Self {
        for part in selection.parts() {
            self = match part {
                SelectionPart::Verse(v) => self.add_verse(v),
                SelectionPart::Passage(p) => self.add_passage(p),
            };
        }
        self
    }

    pub fn build(self) -> Result<ScriptureSelectionRef, String> {
        // TODO: should I fail if there are no selections?
        ScriptureSelectionRef::new(self.selection)
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
        } else if self.start.verse.book == self.end.verse.book {
            write!(
                f,
                "{}:{}{}-{}{}",
                self.start.verse.chapter,
                self.start.verse.number,
                self.start
                    .verse_part
                    .map_or("".to_string(), |p| format!("{p}")),
                self.end.verse.number,
                self.end
                    .verse_part
                    .map_or("".to_string(), |p| format!("{p}")),
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

impl TryFrom<ScriptureRef> for ScriptureVerseRef {
    type Error = String;

    fn try_from(value: ScriptureRef) -> Result<Self, Self::Error> {
        match value {
            ScriptureRef::Verse(v) => Ok(v),
            _ => Err("not a verse ref".to_string()),
        }
    }
}

impl std::str::FromStr for ScriptureRef {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use crate::parser::{Parser, context::interpret};

        let mut parser = Parser::new(s);
        let ast = parser.parse().map_err(|e| e.to_string())?;
        interpret(ast)
    }
}
