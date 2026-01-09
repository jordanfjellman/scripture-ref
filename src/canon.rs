use crate::Book;

#[derive(Debug, Clone, Copy)]
pub(crate) struct ProtestantCanon;

pub(crate) trait Canonical {
    const ORDERED_BOOKS: &'static [Book];

    fn ordered_books(&self) -> &'static [Book];
    fn book_position(&self, book: Book) -> Option<u8>;
    fn book_at_position(&self, position: u8) -> Option<Book>;
    // fn to_canon(&self) -> Canon;
}

impl Canonical for ProtestantCanon {
    const ORDERED_BOOKS: &'static [Book] = &[
        Book::Genesis,
        Book::Exodus,
        Book::FirstKings,
        Book::SongOfSongs,
        Book::Obadiah,
        Book::Matthew,
        Book::Revelation,
    ];

    fn ordered_books(&self) -> &'static [Book] {
        Self::ORDERED_BOOKS
    }

    fn book_position(&self, book: Book) -> Option<u8> {
        self.ordered_books()
            .iter()
            .position(|b| *b == book)
            .map(|b| {
                u8::try_from(b).expect(
                    "position should fit in u8 as it is bounded by the number of existing books",
                )
            })
    }

    fn book_at_position(&self, position: u8) -> Option<Book> {
        self.ordered_books().get(position as usize).map(|b| *b)
    }

    // fn to_canon(&self) -> Canon {
    //     Canon::Protestant
    // }
}

pub(crate) struct InCanon<'c, T, C: Canonical> {
    pub(crate) inner: T,
    pub(crate) canon: &'c C,
}

impl<'c, T, C: Canonical> InCanon<'c, T, C> {
    pub fn new(inner: T, canon: &'c C) -> Self {
        Self { inner, canon }
    }
}
