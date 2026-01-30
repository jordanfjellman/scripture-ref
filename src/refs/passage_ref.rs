use crate::bvc::{Chapter, VerseNumber};

use super::verse_ref::ScriptureVerseRef;

#[derive(Debug, Clone)]
pub struct ScripturePassageRef {
    start: ScriptureVerseRef,
    end: ScriptureVerseRef,
}

impl ScripturePassageRef {
    pub(crate) fn new(start: ScriptureVerseRef, end: ScriptureVerseRef) -> Result<Self, String> {
        Ok(Self { start, end })
    }

    pub(crate) fn builder() -> ScripturePassageRefBuilder {
        ScripturePassageRefBuilder::default()
    }
}

impl std::fmt::Display for ScripturePassageRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: optimize, probably done using ref ids
        if self.start.to_verse() == self.end.to_verse() {
            write!(f, "{}", self.start)
        } else if self.start.to_verse().book == self.end.to_verse().book {
            write!(
                f,
                "{}:{}{}-{}{}",
                self.start.to_verse().chapter,
                self.start.to_verse().number,
                self.start
                    .to_verse_part()
                    .map_or("".to_string(), |p| format!("{p}")),
                self.end.to_verse().number,
                self.end
                    .to_verse_part()
                    .map_or("".to_string(), |p| format!("{p}")),
            )
        } else {
            write!(f, "{}-{}", self.start, self.end)
        }
    }
}

#[derive(Debug)]
pub(crate) struct ScripturePassageRefBuilder {
    start: Option<ScriptureVerseRef>,
    end: Option<ScriptureVerseRef>,
}

impl ScripturePassageRefBuilder {
    pub(crate) fn new() -> Self {
        Self {
            start: None,
            end: None,
        }
    }

    pub(crate) fn start_at(mut self, verse: ScriptureVerseRef) -> Self {
        self.start = Some(verse);
        self
    }

    pub(crate) fn end_at(mut self, verse: ScriptureVerseRef) -> Self {
        self.end = Some(verse);
        self
    }

    pub(crate) fn build(&self) -> Result<ScripturePassageRef, String> {
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

impl std::default::Default for ScripturePassageRefBuilder {
    fn default() -> Self {
        Self::new()
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
