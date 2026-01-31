mod passage_ref;
mod selection_ref;
mod verse_ref;

use crate::refs::selection_ref::{IntoSelectionParts, SelectionPart};

pub use self::{
    passage_ref::ScripturePassageRef, selection_ref::ScriptureSelectionRef,
    verse_ref::ScriptureVerseRef,
};
use std::str::FromStr;

#[derive(Debug)]
pub enum ScriptureRef {
    Verse(ScriptureVerseRef),
    Passage(ScripturePassageRef),
    Selection(ScriptureSelectionRef),
}

impl ScriptureRef {
    pub fn new(string: &str) -> Result<Self, String> {
        Self::from_str(string)
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

impl std::str::FromStr for ScriptureRef {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use crate::parse::parser::{Parser, context::interpret};

        let mut parser = Parser::new(s);
        let ast = parser.parse().map_err(|e| e.to_string())?;
        interpret(ast)
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

impl From<ScriptureSelectionRef> for ScriptureRef {
    fn from(value: ScriptureSelectionRef) -> Self {
        Self::Selection(value)
    }
}

impl IntoSelectionParts for ScriptureRef {
    fn into_parts(self) -> Vec<SelectionPart> {
        match self {
            ScriptureRef::Verse(v) => v.into_parts(),
            ScriptureRef::Passage(p) => p.into_parts(),
            ScriptureRef::Selection(s) => s.into_parts(),
        }
    }
}
