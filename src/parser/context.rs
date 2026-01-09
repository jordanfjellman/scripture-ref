use core::panic;
use std::collections::HashMap;

use crate::domain::book::Book;
use crate::parser::token_tree::Node;

#[derive(Debug, Clone, Copy, PartialEq)]
enum VerseRefNumber {
    Verse(u8),
    Phrase(u8, char),
}

type ChapterRefNumber = u8;

#[derive(Debug, Clone, Copy, PartialEq)]
struct VerseRef(ChapterRefNumber, VerseRefNumber);

#[derive(Debug, Clone, Copy, PartialEq)]
struct PassageRef(VerseRef, VerseRef);

#[derive(Debug, Clone, Copy, PartialEq)]
enum VerseSelection {
    VerseSelection(VerseRef),
    PassageSelection(PassageRef),
}

enum ScriptureRefNumber {
    Chapter(u8),
    Verse(u8, u8),
    Phrase(u8, u8, char),
}

struct ScriptureRefSelectionBuilder {
    book: Option<Book>,
}

impl ScriptureRefSelectionBuilder {
    fn new() -> Self {
        Self {
            book: None,
            // scripture: Vec::new(),
        }
    }

    fn with_book(mut self, book: Book) -> Self {
        self.book = Some(book);
        self
    }

    // fn add_scripture(mut self, scripture: ScriptureRef) -> Self {
    //     self.scripture.push(scripture);
    //     self
    // }
}

// impl TryFrom<ScriptureRefSelectionBuilder> for ScriptureRefSelection {
//     type Error = String;
//
//     fn try_from(builder: ScriptureRefSelectionBuilder) -> Result<Self, Self::Error> {
//         todo!()
//     }
// }

// struct VerseRefNumber(ScriptureRefNumber);
// struct PassageRefNumber(ScriptureRefNumber, ScriptureRefNumber);

#[derive(Debug)]
enum State {
    Initial,
    Operating,
    Done,
}

#[derive(Debug)]
pub struct ParsedScriptureRef {
    pub book: Option<Book>,
    // selection is a map of chapter to verse,passages
    pub verse_selection: HashMap<u8, Vec<VerseSelection>>,
}

impl ParsedScriptureRef {
    fn new() -> Self {
        Self {
            book: None,
            verse_selection: HashMap::new(),
        }
    }

    fn set_book(&mut self, book: Book) {
        self.book = Some(book)
    }

    fn add_verse_selection(&mut self, chapter: &u8, selection: VerseSelection) {
        self.verse_selection
            .entry(*chapter)
            .and_modify(|v| v.push(selection))
            .or_insert_with(|| vec![selection]);
    }
}

#[derive(Debug)]
pub struct Context {
    state: State,
    pub result: ParsedScriptureRef,
}

impl Context {
    pub fn new() -> Self {
        Self {
            state: State::Initial,
            result: ParsedScriptureRef::new(),
        }
    }

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

    pub fn transition(&mut self, node: Node) -> Result<(), miette::Error> {
        match self.state {
            State::Initial => self.process_node(node),
            State::Operating => {
                self.state = State::Done;
                Ok(())
            }
            State::Done => Ok(()), // stop processing
        }
    }

    fn process_node(&mut self, node: Node) -> Result<(), miette::Error> {
        match node {
            Node::And(_, _) => todo!(),
            Node::Book(book) => {
                self.state = State::Operating;
                self.result.set_book(book);
                Ok(())
            }
            Node::InBook(book, _) => {
                self.state = State::Operating;
                self.result.set_book(book);
                Ok(())
            }
            Node::InChapter(chapter, verse_node) => self.process_verse_node(chapter, *verse_node),
            Node::Through(lhs, rhs) => {
                self.process_node(*lhs);
                self.process_node(*rhs);
                Ok(())
            }
            Node::Select(_, _) => todo!(),
            Node::Number(_) => todo!(),
            Node::Nil => todo!(),
        }
    }

    fn process_verse_node(&mut self, chapter: u8, node: Node) -> Result<(), miette::Error> {
        match node {
            Node::And(first, remaining) => match (*first, *remaining) {
                // TODO: Does joining these improve performance/maintainabilty at the expense of
                // easy sorting?
                (Node::Number(v), rhs) => {
                    self.result.add_verse_selection(
                        &chapter,
                        VerseSelection::VerseSelection(VerseRef(chapter, VerseRefNumber::Verse(v))),
                    );
                    self.process_verse_node(chapter, rhs)?;
                    Ok(())
                }
                (lhs, Node::Number(v)) => {
                    self.process_verse_node(chapter, lhs)?;
                    self.result.add_verse_selection(
                        &chapter,
                        VerseSelection::VerseSelection(VerseRef(chapter, VerseRefNumber::Verse(v))),
                    );
                    Ok(())
                }
                (lhs, rhs) => {
                    self.process_verse_node(chapter, lhs)?;
                    self.process_verse_node(chapter, rhs)?;
                    Ok(())
                }
            },
            Node::InChapter(_, _) => todo!(),
            Node::Through(start, end) => match (*start, *end) {
                (Node::Number(s), Node::Number(e)) => {
                    self.result.add_verse_selection(
                        &chapter,
                        VerseSelection::PassageSelection(PassageRef(
                            VerseRef(chapter, VerseRefNumber::Verse(s)),
                            VerseRef(chapter, VerseRefNumber::Verse(e)),
                        )),
                    );
                    Ok(())
                }
                _ => panic!("must have start and end numbers"),
            },
            Node::Select(_, _) => todo!(),
            Node::Number(verse_number) => {
                self.result.add_verse_selection(
                    &chapter,
                    VerseSelection::VerseSelection(VerseRef(
                        chapter,
                        VerseRefNumber::Verse(verse_number),
                    )),
                );
                Ok(())
            }
            other => {
                miette::bail!("cannot process {other} as a verse node");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{
        context::{PassageRef, VerseRef, VerseRefNumber, VerseSelection},
        token_tree::Node,
    };

    use super::Context;

    #[test]
    fn process_in_chapter_node_with_single_verse() {
        let ast = Node::InChapter(1, Box::new(Node::Number(1)));
        let mut context = Context::new();
        context.transition(ast).expect("failed to transition ast");
        assert_eq!(
            context.result.verse_selection.get(&1),
            Some(&vec![VerseSelection::VerseSelection(VerseRef(
                1,
                VerseRefNumber::Verse(1)
            ))])
        );
    }

    #[test]
    fn process_in_chapter_node_with_consecutive_verses() {
        let ast = Node::InChapter(
            1,
            Box::new(Node::Through(
                Box::new(Node::Number(1)),
                Box::new(Node::Number(3)),
            )),
        );
        let mut context = Context::new();
        context.transition(ast).expect("failed to transition ast");
        assert_eq!(
            context.result.verse_selection.get(&1),
            Some(&vec![VerseSelection::PassageSelection(PassageRef(
                VerseRef(1, VerseRefNumber::Verse(1)),
                VerseRef(1, VerseRefNumber::Verse(3)),
            ))])
        );
    }

    #[test]
    fn process_in_chapter_node_with_nonconsecutive_verses() {
        let ast = Node::InChapter(
            1,
            Box::new(Node::And(
                Box::new(Node::Number(1)),
                Box::new(Node::Number(3)),
            )),
        );
        let mut context = Context::new();
        context.transition(ast).expect("failed to transition ast");
        assert_eq!(
            context.result.verse_selection.get(&1),
            Some(&vec![
                VerseSelection::VerseSelection(VerseRef(1, VerseRefNumber::Verse(1))),
                VerseSelection::VerseSelection(VerseRef(1, VerseRefNumber::Verse(3)))
            ])
        );
    }

    #[test]
    fn process_in_chapter_node_with_nonconsecutive_and_consecutive_verses() {
        let ast = Node::InChapter(
            1,
            Box::new(Node::And(
                Box::new(Node::Number(1)),
                Box::new(Node::Through(
                    Box::new(Node::Number(3)),
                    Box::new(Node::Number(5)),
                )),
            )),
        );
        let mut context = Context::new();
        context.transition(ast).expect("failed to transition ast");
        assert_eq!(
            context.result.verse_selection.get(&1),
            Some(&vec![
                VerseSelection::VerseSelection(VerseRef(1, VerseRefNumber::Verse(1))),
                VerseSelection::PassageSelection(PassageRef(
                    VerseRef(1, VerseRefNumber::Verse(3)),
                    VerseRef(1, VerseRefNumber::Verse(5))
                ))
            ])
        );
    }

    #[test]
    fn process_in_chapter_node_with_consecutive_and_nonconsecutive_verses() {
        let ast = Node::InChapter(
            1,
            Box::new(Node::And(
                Box::new(Node::Through(
                    Box::new(Node::Number(1)),
                    Box::new(Node::Number(3)),
                )),
                Box::new(Node::Number(5)),
            )),
        );
        let mut context = Context::new();
        context.transition(ast).expect("failed to transition ast");
        assert_eq!(
            context.result.verse_selection.get(&1),
            Some(&vec![
                VerseSelection::PassageSelection(PassageRef(
                    VerseRef(1, VerseRefNumber::Verse(1)),
                    VerseRef(1, VerseRefNumber::Verse(3))
                )),
                VerseSelection::VerseSelection(VerseRef(1, VerseRefNumber::Verse(5))),
            ])
        );
    }

    #[test]
    fn process_consecutive_verses_through_multiple_chapters() {
        let ast = Node::Through(
            Box::new(Node::InChapter(1, Box::new(Node::Number(1)))),
            Box::new(Node::InChapter(2, Box::new(Node::Number(2)))),
        );
        let mut context = Context::new();
        context.transition(ast).expect("failed to transition ast");
        assert_eq!(
            context.result.verse_selection.get(&1),
            Some(&vec![VerseSelection::PassageSelection(PassageRef(
                VerseRef(1, VerseRefNumber::Verse(1)),
                VerseRef(2, VerseRefNumber::Verse(2))
            )),])
        );
    }
}
