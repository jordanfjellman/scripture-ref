/// The books of the Bible.
///
/// The IDs of the books are arbitrary, but PERMANENT. Once assigned, a book's ID should never
/// change. This serves as a "primary key" for the book. By convention, the IDs of the first 66
/// books match the default, protestant canon.
#[derive(scripture_ref_derive::Book, Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub(crate) enum Book {
    #[chapters = "50"]
    #[verses = "31,25,24,26,32,22,24,22,29,32,32,20,18,24,21,16,27,33,38,18,34,24,20,67,34,35,46,22,35,43,55,32,20,31,29,43,36,30,23,23,57,38,34,34,28,34,31,22,33,26"]
    #[abbreviations = "gen,ge,gn"]
    Genesis = 1,

    #[chapters = "40"]
    #[verses = "23,35,29,15,33,34,28,23,23,35,35,27,22,22,25,33,22,24,19,16,31,21,15,22,29,22,31,29,20,23,28,20,18,23,16,31,23,17,22,16"]
    #[abbreviations = "exod,ex,exo"]
    Exodus = 2,

    #[chapters = "22"]
    #[verses = "53,46,28,20,32,38,51,66,28,29,43,33,34,31,34,34,24,46,21,43,29,54"]
    #[canonical_name = "1 Kings"]
    #[abbreviations = "1 kgs,1 kg,1ki"]
    FirstKings = 11,

    #[chapters = "150"]
    #[verses = "6,11,9,9,13,11,18,10,21,18,7,9,6,7,5,11,15,51,15,10,14,32,6,10,22,11,14,9,11,13,25,11,22,23,28,13,40,23,14,18,14,12,5,27,18,12,10,15,21,23,21,11,7,9,24,14,12,12,18,14,9,13,12,11,14,20,8,36,37,6,24,20,28,23,11,13,21,72,13,20,17,8,19,13,14,17,7,19,53,17,16,16,5,23,11,13,12,9,9,5,8,29,22,35,45,48,43,14,31,7,10,10,9,8,18,19,2,29,176,7,8,9,4,8,5,6,5,6,8,8,3,18,3,3,21,26,9,8,24,14,10,8,12,15,21,10,20,14,9,6"]
    #[abbreviations = "ps,pss"]
    Psalms = 19,

    #[chapters = "8"]
    #[verses = "17,17,13,16,17,15,20,14"]
    #[canonical_name = "Song of Songs"]
    #[abbreviations = "song,song of solomon,sos,canticle of canticles,cant,can"]
    SongOfSongs = 22,

    #[chapters = "1"]
    #[verses = "21"]
    #[abbreviations = "obad,ob"]
    Obadiah = 31,

    #[chapters = "28"]
    #[verses = "25,23,17,25,48,34,29,34,38,42,30,50,58,36,39,28,27,35,30,34,46,46,39,51,46,75,66,20"]
    #[abbreviations = "matt,mat,mt"]
    Matthew = 40,

    #[chapters = "21"]
    #[verses = "51,25,36,54,47,71,53,59,41,42,57,50,38,31,27,33,26,40,42,31,25"]
    #[abbreviations = "jn,jo"]
    John = 43,

    #[chapters = "1"]
    #[verses = "15"]
    #[canonical_name = "3 John"]
    #[abbreviations = "3 jn, 3 jo, 3j"]
    ThirdJohn = 64,

    #[chapters = "22"]
    #[verses = "20,29,22,11,14,17,17,13,21,11,19,18,18,20,8,21,18,24,21,15,27,21"]
    #[abbreviations = "rev,re,rv,revelation to john,apocalypse,apoc,ap"]
    Revelation = 66,
}

impl Book {
    const OLD_TESTAMENT: [Self; 5] = [
        // TODO: update to 39
        Self::Genesis,
        Self::Exodus,
        Self::FirstKings,
        Self::SongOfSongs,
        Self::Obadiah,
    ];

    const NEW_TESTAMENT: [Self; 1] = [Self::Matthew]; // TODO: update to 27

    const BIBLE: [Self; 6] = {
        // TODO: update to 66
        let mut all = [Book::Genesis; 6];
        let mut i = 0;
        while i < Self::OLD_TESTAMENT.len() {
            all[i] = Self::OLD_TESTAMENT[i];
            i += 1;
        }
        let mut j = 0;
        while j < Self::NEW_TESTAMENT.len() {
            all[i] = Self::NEW_TESTAMENT[j];
            i += 1;
            j += 1;
        }
        all
    };

    pub fn old_testament() -> &'static [Self] {
        &Self::OLD_TESTAMENT
    }

    pub fn new_testament() -> &'static [Self] {
        &Self::NEW_TESTAMENT
    }

    pub fn bible() -> &'static [Self] {
        &Self::BIBLE
    }
}

impl std::fmt::Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.canonical_name())
    }
}

impl TryFrom<&str> for Book {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let book = Self::parse(value)?;
        Ok(book.0)
    }
}
