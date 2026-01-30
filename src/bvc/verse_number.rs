#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) struct VerseNumber(u8);

impl VerseNumber {
    pub(crate) fn new(value: u8) -> Result<Self, String> {
        if !(1u8..=176u8).contains(&value) {
            Err(format!(
                "verse {value} out of range; must be positive and not greater than 176"
            ))
        } else {
            Ok(VerseNumber(value))
        }
    }

    pub(crate) fn get(&self) -> u8 {
        self.0
    }
}

impl std::default::Default for VerseNumber {
    fn default() -> Self {
        Self(1)
    }
}

impl std::fmt::Display for VerseNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get())
    }
}

impl TryFrom<u8> for VerseNumber {
    type Error = String;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        VerseNumber::new(value)
    }
}

impl TryFrom<&str> for VerseNumber {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let num = value.parse::<u8>().map_err(|e| {
            format!(
                "not a valid chapter number: {}; error: {:?}",
                value,
                e.kind()
            )
        })?;
        VerseNumber::try_from(num)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use proptest::prelude::*;

    proptest! {
        #[test]
        fn verse_number_valid_range(n in 1u8..=176u8) {
            let result = VerseNumber::new(n);
            prop_assert!(result.is_ok());
            prop_assert_eq!(result.unwrap().0, n);
        }

        #[test]
        fn verse_number_invalid_below(n in 0u8..1u8) {
            prop_assert!(VerseNumber::new(n).is_err());
        }

        #[test]
        fn verse_number_invalid_above(n in 177u8..=255u8) {
            prop_assert!(VerseNumber::new(n).is_err());
        }
    }
}
