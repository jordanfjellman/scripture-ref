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

    #[chapters = "27"]
    #[verses = "17,16,17,35,19,30,38,36,24,20,47,8,59,57,33,34,16,30,37,27,24,33,44,23,55,46,34"]
    #[abbreviations = "lev,le,lv"]
    Leviticus = 3,

    #[chapters = "36"]
    #[verses = "54,34,51,49,31,27,89,26,23,36,35,16,33,45,41,50,13,32,22,29,35,41,30,25,18,65,23,31,40,16,54,42,56,29,34,13"]
    #[abbreviations = "num,nu,nm,nb"]
    Numbers = 4,

    #[chapters = "34"]
    #[verses = "46,37,29,49,33,25,26,20,29,22,32,32,18,29,23,22,20,22,21,20,23,30,25,22,19,19,26,68,29,20,30,52,29,12"]
    #[abbreviations = "deut,dt,de"]
    Deuteronomy = 5,

    #[chapters = "24"]
    #[verses = "18,24,17,24,15,27,26,35,27,43,23,24,33,15,63,10,18,28,51,9,45,34,16,33"]
    #[abbreviations = "josh,jos,jsh"]
    Joshua = 6,

    #[chapters = "21"]
    #[verses = "36,23,31,24,31,40,25,35,57,18,40,15,25,20,20,31,13,31,30,48,25"]
    #[abbreviations = "judg,jdg,jg,jdgs"]
    Judges = 7,

    #[chapters = "4"]
    #[verses = "22,23,17,22"]
    #[abbreviations = "ruth,rth,ru"]
    Ruth = 8,

    #[chapters = "31"]
    #[verses = "28,36,21,22,12,21,17,22,27,27,15,25,23,52,35,23,58,30,24,42,15,23,29,22,44,25,12,25,11,31,13"]
    #[canonical_name = "1 Samuel"]
    #[abbreviations = "1 sam,1sam,1 sm,1sm,1 sa,1sa,1 s,1s"]
    FirstSamuel = 9,

    #[chapters = "24"]
    #[verses = "27,32,39,12,25,23,29,18,13,19,27,31,39,33,37,23,29,33,43,26,22,51,39,25"]
    #[canonical_name = "2 Samuel"]
    #[abbreviations = "2 sam,2sam,2 sm,2sm,2 sa,2sa,2 s,2s"]
    SecondSamuel = 10,

    #[chapters = "22"]
    #[verses = "53,46,28,20,32,38,51,66,28,29,43,33,34,31,34,34,24,46,21,43,29,54"]
    #[canonical_name = "1 Kings"]
    #[abbreviations = "1 kgs,1kgs,1 kg,1kg,1 ki,1ki"]
    FirstKings = 11,

    #[chapters = "25"]
    #[verses = "18,25,27,44,27,33,20,29,37,36,21,21,25,29,38,20,41,37,37,21,26,20,37,20,30"]
    #[canonical_name = "2 Kings"]
    #[abbreviations = "2 kgs,2kgs,2 kg,2kg,2 ki,2ki"]
    SecondKings = 12,

    #[chapters = "29"]
    #[verses = "54,55,24,43,26,81,40,40,44,14,47,40,14,17,29,43,27,17,19,8,30,19,32,31,31,32,34,21,30"]
    #[canonical_name = "1 Chronicles"]
    #[abbreviations = "1 chr,1chr,1 ch,1ch,1 chron,1chron"]
    FirstChronicles = 13,

    #[chapters = "36"]
    #[verses = "17,18,17,22,14,42,22,18,31,19,23,16,22,15,19,14,19,34,11,37,20,12,21,27,28,23,9,27,36,27,21,33,25,33,27,23"]
    #[canonical_name = "2 Chronicles"]
    #[abbreviations = "2 chr,2chr,2 ch,2ch,2 chron,2chron"]
    SecondChronicles = 14,

    #[chapters = "10"]
    #[verses = "11,70,13,24,17,22,28,36,15,44"]
    #[abbreviations = "ezra,ezr,ez"]
    Ezra = 15,

    #[chapters = "13"]
    #[verses = "11,20,32,23,19,19,73,18,38,39,36,47,31"]
    #[abbreviations = "neh,ne"]
    Nehemiah = 16,

    #[chapters = "10"]
    #[verses = "22,23,15,17,14,14,10,17,32,3"]
    #[abbreviations = "esth,est,es"]
    Esther = 17,

    #[chapters = "42"]
    #[verses = "22,13,26,21,27,30,21,22,35,22,20,25,28,22,35,22,16,21,29,29,34,30,17,25,6,14,23,28,25,31,40,22,33,37,16,33,24,41,30,24,34,17"]
    #[abbreviations = "job,jb"]
    Job = 18,

    #[chapters = "150"]
    #[verses = "6,12,8,8,12,10,17,9,20,18,7,8,6,7,5,11,15,50,14,9,13,31,6,10,22,12,14,9,11,12,24,11,22,22,28,12,40,22,13,17,13,11,5,26,17,11,9,14,20,23,19,9,6,7,23,13,11,11,17,12,8,12,11,10,13,20,7,35,36,5,24,20,28,23,10,12,20,72,13,19,16,8,18,12,13,17,7,18,52,17,16,15,5,23,11,13,12,9,9,5,8,28,22,35,45,48,43,13,31,7,10,10,9,8,18,19,2,29,176,7,8,9,4,8,5,6,5,6,8,8,3,18,3,3,21,26,9,8,24,13,10,7,12,15,21,10,20,14,9,6"]
    #[abbreviations = "ps,psa,psm,pss,psalm"]
    Psalms = 19,

    #[chapters = "31"]
    #[verses = "33,22,35,27,23,35,27,36,18,32,31,28,25,35,33,33,28,24,29,30,31,29,35,34,28,28,27,28,27,33,31"]
    #[abbreviations = "prov,pro,pr,prv"]
    Proverbs = 20,

    #[chapters = "12"]
    #[verses = "18,26,22,16,20,12,29,17,18,20,10,14"]
    #[abbreviations = "eccl,ecc,ec,qoh"]
    Ecclesiastes = 21,

    #[chapters = "8"]
    #[verses = "17,17,13,16,17,15,20,14"]
    #[canonical_name = "Song of Songs"]
    #[abbreviations = "song,song of solomon,sos,canticle of canticles,cant,can"]
    SongOfSongs = 22,

    #[chapters = "66"]
    #[verses = "31,22,26,6,30,13,25,22,21,34,16,6,22,32,9,14,14,7,25,6,17,25,18,23,12,21,13,29,24,33,9,20,24,17,10,22,38,22,8,31,29,25,28,28,25,13,15,22,26,11,23,15,12,17,13,12,21,14,21,22,11,12,19,12,25,24"]
    #[abbreviations = "isa,is"]
    Isaiah = 23,

    #[chapters = "52"]
    #[verses = "19,37,25,31,31,30,34,22,26,25,23,17,27,22,21,21,27,23,15,18,14,30,40,10,38,24,22,17,32,24,40,44,26,22,19,32,21,28,18,16,18,22,13,30,5,28,7,47,39,46,64,34"]
    #[abbreviations = "jer,je,jr"]
    Jeremiah = 24,

    #[chapters = "5"]
    #[verses = "22,22,66,22,22"]
    #[abbreviations = "lam,la"]
    Lamentations = 25,

    #[chapters = "48"]
    #[verses = "28,10,27,17,17,14,27,18,11,22,25,28,23,23,8,63,24,32,14,49,32,31,49,27,17,21,36,26,21,26,18,32,33,31,15,38,28,23,29,49,26,20,27,31,25,24,23,35"]
    #[abbreviations = "ezek,eze,ezk"]
    Ezekiel = 26,

    #[chapters = "12"]
    #[verses = "21,49,30,37,31,28,28,27,27,21,45,13"]
    #[abbreviations = "dan,da,dn"]
    Daniel = 27,

    #[chapters = "14"]
    #[verses = "11,23,5,19,15,11,16,14,17,15,12,14,16,9"]
    #[abbreviations = "hos,ho"]
    Hosea = 28,

    #[chapters = "3"]
    #[verses = "20,32,21"]
    #[abbreviations = "joel,jl"]
    Joel = 29,

    #[chapters = "9"]
    #[verses = "15,16,15,13,27,14,17,14,15"]
    #[abbreviations = "amos,am"]
    Amos = 30,

    #[chapters = "1"]
    #[verses = "21"]
    #[abbreviations = "obad,ob"]
    Obadiah = 31,

    #[chapters = "4"]
    #[verses = "17,10,10,11"]
    #[abbreviations = "jonah,jon,jnh"]
    Jonah = 32,

    #[chapters = "7"]
    #[verses = "16,13,12,13,15,16,20"]
    #[abbreviations = "mic,mc"]
    Micah = 33,

    #[chapters = "3"]
    #[verses = "15,13,19"]
    #[abbreviations = "nah,na"]
    Nahum = 34,

    #[chapters = "3"]
    #[verses = "17,20,19"]
    #[abbreviations = "hab,hb"]
    Habakkuk = 35,

    #[chapters = "3"]
    #[verses = "18,15,20"]
    #[abbreviations = "zeph,zep,zp"]
    Zephaniah = 36,

    #[chapters = "2"]
    #[verses = "15,23"]
    #[abbreviations = "hag,hg"]
    Haggai = 37,

    #[chapters = "14"]
    #[verses = "21,13,10,14,11,15,14,23,17,12,17,14,9,21"]
    #[abbreviations = "zech,zec,zc"]
    Zechariah = 38,

    #[chapters = "4"]
    #[verses = "14,17,18,6"]
    #[abbreviations = "mal,ml"]
    Malachi = 39,

    #[chapters = "28"]
    #[verses = "25,23,17,25,48,34,29,34,38,42,30,50,58,36,39,28,27,35,30,34,46,46,39,51,46,75,66,20"]
    #[abbreviations = "matt,mat,mt"]
    Matthew = 40,

    #[chapters = "16"]
    #[verses = "45,28,35,41,43,56,37,38,50,52,33,44,37,72,47,20"]
    #[abbreviations = "mark,mk,mr,mrk"]
    Mark = 41,

    #[chapters = "24"]
    #[verses = "80,52,38,44,39,49,50,56,62,42,54,59,35,35,32,31,37,43,48,47,38,71,56,53"]
    #[abbreviations = "luke,lk,lu"]
    Luke = 42,

    #[chapters = "21"]
    #[verses = "51,25,36,54,47,71,53,59,41,42,57,50,38,31,27,33,26,40,42,31,25"]
    #[abbreviations = "jn,jo"]
    John = 43,

    #[chapters = "28"]
    #[verses = "26,47,26,37,42,15,60,40,43,48,30,25,52,28,41,40,34,28,41,38,40,30,35,27,27,32,44,31"]
    #[abbreviations = "ac,act"]
    Acts = 44,

    #[chapters = "16"]
    #[verses = "32,29,31,25,21,23,25,39,33,21,36,21,14,23,33,27"]
    #[abbreviations = "rom,ro,rm"]
    Romans = 45,

    #[chapters = "16"]
    #[verses = "31,16,23,21,13,20,40,13,27,33,34,31,13,40,58,24"]
    #[canonical_name = "1 Corinthians"]
    //TODO: #[abbreviations = "1 cor,1cor,1 co,1co,1 c,1c"]
    #[abbreviations = "1 cor,1cor,1 co,1co,1 c"]
    FirstCorinthians = 46,

    #[chapters = "13"]
    #[verses = "24,17,18,18,21,18,16,24,15,18,33,21,14"]
    #[canonical_name = "2 Corinthians"]
    //TODO: #[abbreviations = "2 cor,2cor,2 co,2co,2 c,2c"]
    #[abbreviations = "2 cor,2cor,2 co,2co,2 c"]
    SecondCorinthians = 47,

    #[chapters = "6"]
    #[verses = "24,21,29,31,26,18"]
    #[abbreviations = "gal,ga"]
    Galatians = 48,

    #[chapters = "6"]
    #[verses = "23,22,21,32,33,24"]
    #[abbreviations = "eph,ep,ephes"]
    Ephesians = 49,

    #[chapters = "4"]
    #[verses = "30,30,21,23"]
    #[abbreviations = "phil,php,pp,phi"]
    Philippians = 50,

    #[chapters = "4"]
    #[verses = "29,23,25,18"]
    #[abbreviations = "col,co"]
    Colossians = 51,

    #[chapters = "5"]
    #[verses = "10,20,13,18,28"]
    #[canonical_name = "1 Thessalonians"]
    #[abbreviations = "1 thess,1thess,1 th,1th,1 ts,1ts"]
    FirstThessalonians = 52,

    #[chapters = "3"]
    #[verses = "12,17,18"]
    #[canonical_name = "2 Thessalonians"]
    #[abbreviations = "2 thess,2thess,2 th,2th,2 ts,2ts"]
    SecondThessalonians = 53,

    #[chapters = "6"]
    #[verses = "20,15,16,16,25,21"]
    #[canonical_name = "1 Timothy"]
    #[abbreviations = "1 tim,1tim,1 ti,1ti,1 tm,1tm"]
    FirstTimothy = 54,

    #[chapters = "4"]
    #[verses = "18,26,17,22"]
    #[canonical_name = "2 Timothy"]
    #[abbreviations = "2 tim,2tim,2 ti,2ti,2 tm,2tm"]
    SecondTimothy = 55,

    #[chapters = "3"]
    #[verses = "16,15,15"]
    #[abbreviations = "titus,tit,ti"]
    Titus = 56,

    #[chapters = "1"]
    #[verses = "25"]
    #[abbreviations = "phlm,phm,pm"]
    Philemon = 57,

    #[chapters = "13"]
    #[verses = "14,18,19,16,14,20,28,13,28,39,40,29,25"]
    #[abbreviations = "heb,he"]
    Hebrews = 58,

    #[chapters = "5"]
    #[verses = "27,26,18,17,20"]
    #[abbreviations = "jas,jm,jam"]
    James = 59,

    #[chapters = "5"]
    #[verses = "25,25,22,19,14"]
    #[canonical_name = "1 Peter"]
    #[abbreviations = "1 pet,1pet,1 pe,1pe,1 pt,1pt,1 p,1p"]
    FirstPeter = 60,

    #[chapters = "3"]
    #[verses = "21,22,18"]
    #[canonical_name = "2 Peter"]
    #[abbreviations = "2 pet,2pet,2 pe,2pe,2 pt,2pt,2 p,2p"]
    SecondPeter = 61,

    #[chapters = "5"]
    #[verses = "10,29,24,21,21"]
    #[canonical_name = "1 John"]
    #[abbreviations = "1 john,1john,1 jn,1jn,1 jo,1jo,1 j,1j"]
    FirstJohn = 62,

    #[chapters = "1"]
    #[verses = "13"]
    #[canonical_name = "2 John"]
    #[abbreviations = "2 john,2john,2 jn,2jn,2 jo,2jo,2 j,2j"]
    SecondJohn = 63,

    #[chapters = "1"]
    #[verses = "14"]
    #[canonical_name = "3 John"]
    #[abbreviations = "3 john,3john,3 jn,3jn,3 jo,3jo,3 j,3j"]
    ThirdJohn = 64,

    #[chapters = "1"]
    #[verses = "25"]
    #[abbreviations = "jude,jud,jd"]
    Jude = 65,

    #[chapters = "22"]
    #[verses = "20,29,22,11,14,17,17,13,21,11,19,18,18,20,8,21,18,24,21,15,27,21"]
    #[abbreviations = "rev,re,rv,revelation to john,apocalypse,apoc,ap"]
    Revelation = 66,
}

impl Book {
    const ALL: [Self; 66] = [
        Self::Genesis,
        Self::Exodus,
        Self::Leviticus,
        Self::Numbers,
        Self::Deuteronomy,
        Self::Joshua,
        Self::Judges,
        Self::Ruth,
        Self::FirstSamuel,
        Self::SecondSamuel,
        Self::FirstKings,
        Self::SecondKings,
        Self::FirstChronicles,
        Self::SecondChronicles,
        Self::Ezra,
        Self::Nehemiah,
        Self::Esther,
        Self::Job,
        Self::Psalms,
        Self::Proverbs,
        Self::Ecclesiastes,
        Self::SongOfSongs,
        Self::Isaiah,
        Self::Jeremiah,
        Self::Lamentations,
        Self::Ezekiel,
        Self::Daniel,
        Self::Hosea,
        Self::Joel,
        Self::Amos,
        Self::Obadiah,
        Self::Jonah,
        Self::Micah,
        Self::Nahum,
        Self::Habakkuk,
        Self::Zephaniah,
        Self::Haggai,
        Self::Zechariah,
        Self::Malachi,
        Self::Matthew,
        Self::Mark,
        Self::Luke,
        Self::John,
        Self::Acts,
        Self::Romans,
        Self::FirstCorinthians,
        Self::SecondCorinthians,
        Self::Galatians,
        Self::Ephesians,
        Self::Philippians,
        Self::Colossians,
        Self::FirstThessalonians,
        Self::SecondThessalonians,
        Self::FirstTimothy,
        Self::SecondTimothy,
        Self::Titus,
        Self::Philemon,
        Self::Hebrews,
        Self::James,
        Self::FirstPeter,
        Self::SecondPeter,
        Self::FirstJohn,
        Self::SecondJohn,
        Self::ThirdJohn,
        Self::Jude,
        Self::Revelation,
    ];

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

    pub fn all() -> &'static [Self] {
        &Self::ALL
    }

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
