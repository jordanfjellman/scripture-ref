use crate::domain::{
    book::Book,
    scripture_reference::{Reference, ScriptureReference},
};

// Reference: Single Verse or Verse Subpart
// ReferenceRange: Continuous Verses or Verse
// Passage: Collection of Reference[] or ReferenceRange[]
// Selection: Collection of Verses or Passages

// ";" seems to separate passages
// "," seems to separate

enum ParseState {
    ExpectingBook,
    ExpectingChapter,
    ExpectingVerse,
}

struct ScripturePassageParser {
    state: ParseState,
    builder: ScripturePassageBuilder,
}

struct ScripturePassageBuilder {
    references: std::rc::Rc<ScriptureReference>,
}

struct ScriptureReferenceBuilder {
    kind: Reference,
    // book: Option<Book>,
    // first_chapter: Option<u8>,
    // first_verse: Option<u8>,
    // through_chapter: Option<u8>,
    // through_verse: Option<u8>,
}
