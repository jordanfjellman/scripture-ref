use crate::bvc::{Book, Chapter, ChapterNumber, VerseNumber};
use crate::parse::parser::token_tree::Node;
use crate::refs::{ScripturePassageRef, ScriptureRef, ScriptureSelectionRef, ScriptureVerseRef};

#[derive(Debug, Clone, Copy)]
struct InterpreterContext {
    book: Option<Book>,
    chapter: Option<ChapterNumber>,
    verse: Option<VerseNumber>,
}

impl InterpreterContext {
    fn new() -> Self {
        Self {
            book: None,
            chapter: None,
            verse: None,
        }
    }

    fn from_book(book: Book) -> Self {
        Self {
            book: Some(book),
            chapter: None,
            verse: None,
        }
    }

    fn with_chapter(mut self, chapter: ChapterNumber) -> Self {
        self.chapter = Some(chapter);
        self
    }

    fn with_verse(mut self, verse: VerseNumber) -> Self {
        self.verse = Some(verse);
        self
    }

    fn reset_chapter(mut self) -> Self {
        self.chapter = None;
        self.reset_verse()
    }

    fn reset_verse(mut self) -> Self {
        self.verse = None;
        self
    }
}

impl std::default::Default for InterpreterContext {
    fn default() -> Self {
        Self::new()
    }
}

pub(crate) fn interpret(node: Node) -> Result<ScriptureRef, String> {
    interpret_with_context(node, &InterpreterContext::default())
}

fn interpret_with_context(node: Node, ctx: &InterpreterContext) -> Result<ScriptureRef, String> {
    match node {
        Node::InBook(book, inner) => interpret_in_book(book, *inner),
        Node::InChapter(chapter, inner) => interpret_in_chapter(chapter, *inner, ctx),
        Node::InVerse(verse, inner) => interpret_in_verse(verse, *inner, ctx),
        Node::Number(verse_number) => interpret_number(verse_number, ctx),
        Node::And(left, right) => interpret_and(*left, *right, ctx),
        Node::Select(left, right) => interpret_select(*left, *right, ctx),
        Node::Through(left, right) => interpret_through(*left, *right, ctx),
        Node::Following(verse) => interpret_following(*verse, ctx),
        Node::Nil => interpret_nil(ctx),
        Node::Book(book) => interpret_book(book),
        Node::VersePart(part) => interpret_verse_part(part, ctx),
    }
}

fn interpret_in_book(book: Book, inner: Node) -> Result<ScriptureRef, String> {
    let new_ctx = InterpreterContext::from_book(book);

    if book.chapter_count() == 1 {
        match &inner {
            Node::Number(v) => {
                return ScriptureVerseRef::builder()
                    .book(book)
                    .chapter(ChapterNumber::default())
                    .try_verse(*v)?
                    .build()
                    .map(|v| v.into());
            }
            Node::Through(left, right) => {
                if let (Node::Number(start), Node::Number(end)) = (left.as_ref(), right.as_ref()) {
                    return ScripturePassageRef::builder()
                        .start_at(
                            ScriptureVerseRef::builder()
                                .book(book)
                                .chapter(ChapterNumber::default())
                                .try_verse(*start)?
                                .build()
                                .map(|v| v.into())?,
                        )
                        .end_at(
                            ScriptureVerseRef::builder()
                                .book(book)
                                .chapter(ChapterNumber::default())
                                .try_verse(*end)?
                                .build()
                                .map(|v| v.into())?,
                        )
                        .build()
                        .map(|p| p.into());
                }
            }
            Node::Select(_, _) | Node::And(_, _) | Node::Following(_) => {
                let ctx = new_ctx.with_chapter(ChapterNumber::default());
                return interpret_with_context(inner, &ctx);
            }
            _ => {}
        }
    }

    interpret_with_context(inner, &new_ctx)
}

fn interpret_in_chapter(
    chapter: ChapterNumber,
    inner: Node,
    ctx: &InterpreterContext,
) -> Result<ScriptureRef, String> {
    let new_ctx = ctx.with_chapter(chapter);
    interpret_with_context(inner, &new_ctx)
}

fn interpret_in_verse(
    verse: VerseNumber,
    inner: Node,
    ctx: &InterpreterContext,
) -> Result<ScriptureRef, String> {
    let new_ctx = ctx.with_verse(verse);
    interpret_with_context(inner, &new_ctx)
}

fn interpret_number(number: u8, ctx: &InterpreterContext) -> Result<ScriptureRef, String> {
    let book = ctx.book.ok_or("no book in context")?;
    match ctx.chapter {
        Some(chapter) => ScriptureVerseRef::builder()
            .book(book)
            .chapter(chapter)
            .try_verse(number)?
            .build()
            .map(|v| v.into()),
        None => ScripturePassageRef::builder()
            .start_at(
                ScriptureVerseRef::builder()
                    .book(book)
                    .try_chapter(number)?
                    .verse(VerseNumber::default())
                    .build()
                    .map(|v| v.into())?,
            )
            .end_at(
                ScriptureVerseRef::builder()
                    .book(book)
                    .try_chapter(number)?
                    .try_verse(book.max_verses_in_chapter(number)?)?
                    .build()
                    .map(|v| v.into())?,
            )
            .build()
            .map(|p| p.into()),
    }
}

fn interpret_verse_part(part: u8, ctx: &InterpreterContext) -> Result<ScriptureRef, String> {
    let book = ctx.book.ok_or("no book in context")?;
    let chapter = ctx.chapter.ok_or("no chapter in context")?;
    let verse = ctx.verse.ok_or("no verse in context")?;
    ScriptureVerseRef::builder()
        .book(book)
        .chapter(chapter)
        .verse(verse)
        .try_verse_part(part)?
        .build()
        .map(|v| v.into())
}

// semicolon
fn interpret_and(
    left: Node,
    right: Node,
    ctx: &InterpreterContext,
) -> Result<ScriptureRef, String> {
    let left = interpret_with_context(left, &ctx)?;
    let right_ctx = ctx.reset_chapter();
    let right = interpret_with_context(right, &right_ctx)?;
    ScriptureSelectionRef::builder()
        .add_selection_part(left)
        .add_selection_part(right)
        .build()
        .map(|s| s.into())
}

// comma
fn interpret_select(
    left: Node,
    right: Node,
    ctx: &InterpreterContext,
) -> Result<ScriptureRef, String> {
    let right_ctx = match (&left, &right) {
        (Node::InVerse(verse, _), Node::VersePart(_)) => ctx.with_verse(*verse),
        (Node::InChapter(chapter, inner), Node::VersePart(_)) => {
            if let Node::InVerse(verse, _) = inner.as_ref() {
                ctx.with_chapter(*chapter).with_verse(*verse)
            } else {
                *ctx
            }
        }
        _ => *ctx,
    };
    let left = interpret_with_context(left, &ctx)?;
    let right = interpret_with_context(right, &right_ctx)?;
    ScriptureSelectionRef::builder()
        .add_selection_part(left)
        .add_selection_part(right)
        .build()
        .map(|s| s.into())
}

// hyphen
fn interpret_through(
    left: Node,
    right: Node,
    ctx: &InterpreterContext,
) -> Result<ScriptureRef, String> {
    let inherited_chapter_ctx = match (&left, &right) {
        (Node::InChapter(chapter, _), Node::Number(_))
        | (Node::InChapter(chapter, _), Node::InVerse(_, _)) => Some(ctx.with_chapter(*chapter)),
        _ => None,
    };
    let inherited_verse_ctx = match (&left, &right) {
        (Node::InVerse(verse, _), Node::VersePart(_)) => {
            let base_ctx = inherited_chapter_ctx.as_ref().unwrap_or(ctx);
            Some(base_ctx.with_verse(*verse))
        }
        (Node::InChapter(chapter, inner), Node::VersePart(_)) => {
            if let Node::InVerse(verse, _) = inner.as_ref() {
                Some(ctx.with_chapter(*chapter).with_verse(*verse))
            } else {
                None
            }
        }
        _ => None,
    };

    let left_ref = interpret_with_context(left.clone(), &ctx)?;

    let right_ref = if let Some(ref v_ctx) = inherited_verse_ctx {
        interpret_with_context(right, v_ctx)?
    } else if let Some(ref c_ctx) = inherited_chapter_ctx {
        interpret_with_context(right, c_ctx)?
    } else {
        interpret_with_context(right, &ctx)?
    };

    ScripturePassageRef::builder()
        .start_at(left_ref.try_into()?)
        .end_at(right_ref.try_into()?)
        .build()
        .map(|p| p.into())
}

// ff
fn interpret_following(verse: Node, ctx: &InterpreterContext) -> Result<ScriptureRef, String> {
    println!("following: {verse:?}");
    let book = ctx.book.ok_or("no book in context")?;
    let chapter = ctx.chapter.ok_or("no chapter in context")?;
    let chapter = Chapter::new(book, chapter)?;

    ScripturePassageRef::builder()
        .start_at(
            ScriptureVerseRef::builder()
                .book(book)
                .chapter(chapter.number)
                .verse(verse.try_into()?)
                .build()?,
        )
        .end_at(
            ScriptureVerseRef::builder()
                .book(book)
                .chapter(chapter.number)
                .verse(chapter.max_verse_count()?.try_into()?)
                .build()?,
        )
        .build()
        .map(|p| p.into())
}

fn interpret_nil(_ctx: &InterpreterContext) -> Result<ScriptureRef, String> {
    // TODO: Book, but had no other detail; should be a specific error
    Err("unexpected Nil node".to_string())
}

// standalone book
fn interpret_book(book: Book) -> Result<ScriptureRef, String> {
    Err(format!("unexpected stand alone book node {book}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::parser::Parser;
    fn parse_and_interpret(input: &str) -> Result<ScriptureRef, String> {
        let mut parser = Parser::new(input);
        let ast = parser.parse().map_err(|e| e.to_string())?;
        interpret(ast)
    }

    #[test]
    fn single_verse() {
        let result = parse_and_interpret("Genesis 1:1").unwrap();
        assert_eq!(result.to_string(), "Genesis 1:1");
    }

    #[test]
    fn passage() {
        let result = parse_and_interpret("Genesis 1:1-2").unwrap();
        assert_eq!(result.to_string(), "Genesis 1:1-2");
    }

    #[test]
    fn selection_comma() {
        let result = parse_and_interpret("Genesis 1:1,3").unwrap();
        assert!(matches!(result, ScriptureRef::Selection(_)));
    }

    #[test]
    fn verse_selection_semicolon() {
        let result = parse_and_interpret("Genesis 1:1; 2:3").unwrap();
        assert!(matches!(result, ScriptureRef::Selection(_)));
    }

    #[test]
    fn complex_selection() {
        let result = parse_and_interpret("Genesis 1:1,3; 2:12").unwrap();
        assert!(matches!(result, ScriptureRef::Selection(_)));
    }

    #[test]
    fn cross_chapter_passage() {
        let result = parse_and_interpret("Genesis 1:5-2:3").unwrap();
        assert!(matches!(result, ScriptureRef::Passage(_)));
    }

    #[test]
    fn chapter_only() {
        let result = parse_and_interpret("Genesis 1").unwrap();
        assert!(matches!(result, ScriptureRef::Passage(_)));
    }

    #[test]
    fn following_verses() {
        let result = parse_and_interpret("Genesis 1:5ff").unwrap();
        assert!(matches!(result, ScriptureRef::Passage(_)));
    }

    #[test]
    fn single_chapter_book_verse() {
        let result = parse_and_interpret("Obadiah 5").unwrap();
        assert_eq!(result.to_string(), "Obadiah 1:5");
    }

    #[test]
    fn single_chapter_book_explicit() {
        let result = parse_and_interpret("Obadiah 1:5").unwrap();
        assert_eq!(result.to_string(), "Obadiah 1:5");
    }

    #[test]
    fn single_chapter_book_3john() {
        let result = parse_and_interpret("3 John 5").unwrap();
        assert_eq!(result.to_string(), "3 John 1:5");
    }

    #[test]
    fn verse_with_part() {
        let result = parse_and_interpret("Genesis 1:5a").unwrap();
        assert!(matches!(result, ScriptureRef::Verse(_)));
        assert_eq!(result.to_string(), "Genesis 1:5a");
    }

    #[test]
    fn verse_part_range_same_verse() {
        let result = parse_and_interpret("Genesis 1:5a-b").unwrap();
        assert!(matches!(result, ScriptureRef::Passage(_)));
    }

    #[test]
    fn verse_part_range_cross_verse() {
        let result = parse_and_interpret("Genesis 1:5a-6b").unwrap();
        assert!(matches!(result, ScriptureRef::Passage(_)));
    }

    #[test]
    fn verse_part_to_whole_verse() {
        let result = parse_and_interpret("Genesis 1:5a-6").unwrap();
        assert!(matches!(result, ScriptureRef::Passage(_)));
    }

    #[test]
    fn whole_verse_to_verse_part() {
        let result = parse_and_interpret("Genesis 1:5-6b").unwrap();
        assert!(matches!(result, ScriptureRef::Passage(_)));
    }

    #[test]
    fn verse_part_selection() {
        let result = parse_and_interpret("Genesis 1:5a,c").unwrap();
        assert!(matches!(result, ScriptureRef::Selection(_)));
    }

    #[test]
    fn complex_verse_selection() {
        let result = parse_and_interpret("1 Kings 1:1-2a, 3:4b-5").unwrap();
        assert!(matches!(result, ScriptureRef::Selection(_)));
    }
}
