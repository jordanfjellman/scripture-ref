use crate::bvc::{Book, ChapterNumber, VerseNumber};
use crate::parser::token_tree::Node;
use crate::scripture_ref_builder::{
    ScripturePassageRef, ScriptureRef, ScriptureSelectionRef, ScriptureVerseRef,
};

// IN_BOOK
// |__ Book(john)
// |__ AND               # ;
//     |__ IN_CHAPTER
//     |   |__ Number(2) # chapter
//     |   |__ Number(1) # verse
//     |__ IN_CHAPTER
//         |__ Number(1) # chapter
//         |__ Number(1) # verse
//
// IN_BOOK
// |__ Book(john)
// |__ IN_CHAPTER
//     |__ Number(1)         # chapter
//     |__ SELECT            # ,
//         |__ Number(5)     # verse
//         |__ THROUGH       # -
//             |__ Number(2) # verse
//             |__ Number(1) # verse

#[derive(Debug, Clone, Copy)]
struct InterpreterContext {
    book: Option<Book>,
    chapter: Option<ChapterNumber>,
}

impl InterpreterContext {
    fn new() -> Self {
        Self {
            book: None,
            chapter: None,
        }
    }

    fn from_book(book: Book) -> Self {
        Self {
            book: Some(book),
            chapter: None,
        }
    }

    fn with_chapter(mut self, chapter: ChapterNumber) -> Self {
        self.chapter = Some(chapter);
        self
    }

    fn reset_chapter(mut self) -> Self {
        self.chapter = None;
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
        Node::Number(verse_number) => interpret_number(verse_number, ctx),
        Node::And(left, right) => interpret_and(*left, *right, ctx),
        Node::Select(left, right) => interpret_select(*left, *right, ctx),
        Node::Through(left, right) => interpret_through(*left, *right, ctx),
        Node::Following(following) => interpret_following(*following),
        Node::Nil => interpret_nil(ctx),
        Node::Book(book) => interpret_book(book),
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
    chapter: u8,
    inner: Node,
    ctx: &InterpreterContext,
) -> Result<ScriptureRef, String> {
    println!("interpret_in_chapter {chapter} {inner:?}");
    let chapter_num = ChapterNumber::try_from(chapter)?;
    let new_ctx = ctx.with_chapter(chapter_num);
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
        .add_scripture_ref(left)
        .add_scripture_ref(right)
        .build()
        .map(|s| s.into())
}

// comma
fn interpret_select(
    left: Node,
    right: Node,
    ctx: &InterpreterContext,
) -> Result<ScriptureRef, String> {
    let left = interpret_with_context(left, &ctx)?;
    let right = interpret_with_context(right, &ctx)?;
    ScriptureSelectionRef::builder()
        .add_scripture_ref(left)
        .add_scripture_ref(right)
        .build()
        .map(|s| s.into())
}

// hyphen
fn interpret_through(
    left: Node,
    right: Node,
    ctx: &InterpreterContext,
) -> Result<ScriptureRef, String> {
    let left_ref = interpret_with_context(left.clone(), &ctx)?;

    let right_ref = match (&left, &right) {
        (Node::InChapter(chapter, _), Node::Number(_)) => {
            let chapter_ctx = ctx.with_chapter(ChapterNumber::try_from(*chapter)?);
            interpret_with_context(right, &chapter_ctx)?
        }
        _ => interpret_with_context(right, &ctx)?,
    };

    ScripturePassageRef::builder()
        .start_at(left_ref.try_into()?)
        .end_at(right_ref.try_into()?)
        .build()
        .map(|p| p.into())
}

// ff
fn interpret_following(_node: Node) -> Result<ScriptureRef, String> {
    todo!()
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
    use crate::parser::Parser;
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
        println!("{result:?}");
        assert_eq!(result.to_string(), "Genesis 1:1-2");
    }

    #[test]
    fn selection_comma() {
        let result = parse_and_interpret("Genesis 1:1,3").unwrap();
        // Selection of two verses in same chapter
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
        // Should be passage from 1:1 to 1:31
        assert!(matches!(result, ScriptureRef::Passage(_)));
    }

    #[test]
    fn following_verses() {
        let result = parse_and_interpret("Genesis 1:5ff").unwrap();
        // Should be passage from 1:5 to 1:31
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
}
