/*
    1. Verilen testi miktar ve para birimine ayır.
    2. "42 Lira" -> 42 ve "Lira"
    3. "42 Lira" -> (f64, String)
*/


use std::{error::Error, fmt::Display, num::ParseFloatError}; 




pub fn parse_money(input: &str) -> Result<(f64, String), MoneyError> {
    let pieces: Vec<&str>  = input.split_whitespace().collect();
    // ["42", "lira"]
    if 2 != pieces.len() {
        Err(MoneyError {kind:MoneyErrorKind::ParseError,})
    }
    else {
        let quantity: f64 = pieces[0].parse()?;
        let currency_unit: String = pieces[1].into();
        Ok((quantity, currency_unit))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MoneyError {
    kind: MoneyErrorKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum MoneyErrorKind {
    ParseError,
}


impl Error for MoneyError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self)
    }
}

impl Display for MoneyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            MoneyErrorKind::ParseError => write!(f, "Money parsing error")
        }
    }
}

impl From<ParseFloatError> for MoneyError {
    fn from(_: ParseFloatError) -> Self {
        MoneyError {
            kind: MoneyErrorKind::ParseError,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positive_decimals() {
        let result: Result<(f64, String), MoneyError> = parse_money("42.1 lira");
        assert_eq!(Ok((42.1, "lira".to_string())), result);
    }
    #[test]
    fn negative_decimal() {
        let result: Result<(f64, String), MoneyError> = parse_money("-42.1 lira");
        assert_eq!(Ok((-42.1, "lira".to_string())), result);
    }

    #[test]
    fn positive_integer_number() {
        let result: Result<(f64, String), MoneyError> = parse_money("42.1 lira");
        assert_eq!(Ok((42.1, "lira".to_string())), result);
    }
    #[test]
    fn negative_integer_number() {
        let result: Result<(f64, String), MoneyError> = parse_money("-42.1 lira");
        assert_eq!(Ok((-42.1, "lira".to_string())), result);
    }

    #[test]
    fn single_item() {
        parse_money("42").unwrap();
    }
}
