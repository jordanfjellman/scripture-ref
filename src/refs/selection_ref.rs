use super::passage_ref::ScripturePassageRef;
use super::verse_ref::ScriptureVerseRef;

#[derive(Debug, Clone)]
pub struct ScriptureSelectionRef(Vec<SelectionPart>);

#[derive(Debug, Clone)]
pub enum SelectionPart {
    Verse(ScriptureVerseRef),
    Passage(ScripturePassageRef),
}

impl ScriptureSelectionRef {
    pub(crate) fn new(selection: Vec<SelectionPart>) -> Result<Self, String> {
        Ok(Self(selection))
    }

    pub(crate) fn parts(self) -> Vec<SelectionPart> {
        self.0
    }

    pub(crate) fn builder() -> ScriptureSelectionRefBuilder {
        ScriptureSelectionRefBuilder::new()
    }
}

#[derive(Debug)]
pub(crate) struct ScriptureSelectionRefBuilder {
    selection: Vec<SelectionPart>,
}

pub(crate) trait IntoSelectionParts {
    fn into_parts(self) -> Vec<SelectionPart>;
}

impl ScriptureSelectionRefBuilder {
    pub(crate) fn new() -> Self {
        Self {
            selection: Vec::new(),
        }
    }

    pub(crate) fn add_selection_part<T: IntoSelectionParts>(mut self, item: T) -> Self {
        self.selection.extend(item.into_parts());
        self
    }

    pub(crate) fn add_verse(mut self, verse: ScriptureVerseRef) -> Self {
        self.selection.push(SelectionPart::Verse(verse));
        self
    }

    pub(crate) fn add_passage(mut self, passage: ScripturePassageRef) -> Self {
        self.selection.push(SelectionPart::Passage(passage));
        self
    }

    pub(crate) fn add_selection(mut self, selection: ScriptureSelectionRef) -> Self {
        for part in selection.parts() {
            self = match part {
                SelectionPart::Verse(v) => self.add_verse(v),
                SelectionPart::Passage(p) => self.add_passage(p),
            };
        }
        self
    }

    pub(crate) fn build(self) -> Result<ScriptureSelectionRef, String> {
        // TODO: should I fail if there are no selections?
        ScriptureSelectionRef::new(self.selection)
    }
}

impl IntoSelectionParts for ScriptureVerseRef {
    fn into_parts(self) -> Vec<SelectionPart> {
        vec![SelectionPart::Verse(self)]
    }
}

impl IntoSelectionParts for ScripturePassageRef {
    fn into_parts(self) -> Vec<SelectionPart> {
        vec![SelectionPart::Passage(self)]
    }
}

impl IntoSelectionParts for ScriptureSelectionRef {
    fn into_parts(self) -> Vec<SelectionPart> {
        self.parts()
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
