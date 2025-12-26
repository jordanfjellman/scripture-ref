pub fn parse_u8_from_string(s: &str) -> syn::Result<u8> {
    s.trim()
        .parse::<u8>()
        .map_err(|_| syn::Error::new_spanned(&format!("\"{}\"", s), "expected a valid u8 value"))
}

pub fn parse_u8_array_from_string(s: &str) -> syn::Result<Vec<u8>> {
    s.split(',')
        .map(|part| parse_u8_from_string(part.trim()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_u8_from_string() {
        let result = parse_u8_from_string("24").unwrap();
        assert_eq!(result, 24);
    }

    #[test]
    fn test_parse_u8_from_string_with_whitespace() {
        let result = parse_u8_from_string("  42  ").unwrap();
        assert_eq!(result, 42);
    }

    #[test]
    fn test_parse_u8_from_string_invalid() {
        let result = parse_u8_from_string("not a number");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_u8_from_string_out_of_range() {
        let result = parse_u8_from_string("300");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_u8_array_from_string() {
        let result = parse_u8_array_from_string("24, 25, 26").unwrap();
        assert_eq!(result, vec![24, 25, 26]);
    }

    #[test]
    fn test_parse_u8_array_from_string_single() {
        let result = parse_u8_array_from_string("42").unwrap();
        assert_eq!(result, vec![42]);
    }

    #[test]
    fn test_parse_u8_array_from_string_with_whitespace() {
        let result = parse_u8_array_from_string("  1  ,  2  ,  3  ").unwrap();
        assert_eq!(result, vec![1, 2, 3]);
    }
}
