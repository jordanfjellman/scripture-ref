#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) struct ChapterNumber(u8);

impl ChapterNumber {
    pub(crate) fn new(value: u8) -> Result<Self, String> {
        if !(1u8..=150u8).contains(&value) {
            Err(format!(
                "chapter {value} is out of range; must be positive and not greater than 150"
            ))
        } else {
            Ok(ChapterNumber(value))
        }
    }

    pub(crate) fn get(&self) -> u8 {
        self.0
    }
}

impl std::default::Default for ChapterNumber {
    fn default() -> Self {
        ChapterNumber(1)
    }
}

impl std::fmt::Display for ChapterNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get())
    }
}

impl TryFrom<u8> for ChapterNumber {
    type Error = String;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        ChapterNumber::new(value)
    }
}

impl TryFrom<&str> for ChapterNumber {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let num = value.parse::<u8>().map_err(|e| {
            format!(
                "not a valid chapter number: {}; error: {:?}",
                value,
                e.kind()
            )
        })?;
        ChapterNumber::try_from(num)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use proptest::prelude::*;

    proptest! {
        #[test]
        fn chapter_number_valid_range(n in 1u8..=150u8) {
            let result = ChapterNumber::new(n);
            prop_assert!(result.is_ok());
            prop_assert_eq!(result.unwrap().0, n);
        }

        #[test]
        fn chapter_number_invalid_below(n in 0u8..1u8) {
            prop_assert!(ChapterNumber::new(n).is_err());
        }

        #[test]
        fn chapter_number_invalid_above(n in 151u8..=255u8) {
            prop_assert!(ChapterNumber::new(n).is_err());
        }
    }
}
