#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) struct VersePartLabel(u8);

// TODO: should this be more similar to the Verse type?
impl VersePartLabel {
    pub(crate) fn new(value: u8) -> Result<Self, String> {
        if !(b'a'..=b'd').contains(&value) {
            Err(format!(
                "verse phrase {value} is not valid, must be a single letter from a to d"
            ))
        } else {
            Ok(Self(value))
        }
    }

    pub(crate) fn get(&self) -> u8 {
        self.0
    }

    pub(crate) fn max() -> Self {
        Self(b'd') // TODO: share max logic
    }
}

impl std::fmt::Display for VersePartLabel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_ascii_lowercase() as char)
    }
}

impl TryFrom<u8> for VersePartLabel {
    type Error = String;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        VersePartLabel::new(value)
    }
}
