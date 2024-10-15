/*
    1. Verilen testi miktar ve para birimine ayır.
    2. "42 Lira" -> 42 ve "Lira"
    3. "42 Lira" -> Money
*/


use core::str;
use std::{error::Error, fmt::Display, num::ParseFloatError, str::FromStr}; 
use std::process::Termination;


#[derive(Debug, Clone, PartialEq, Default)]
pub struct Money {
    quantity: f64,
    currency_unit: CurrencyUnit,
}

impl FromStr for Money {
    type Err = MoneyError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let pieces: Vec<&str>  = input.split_whitespace().collect();
    
        match pieces[..] {
            [quantity, currency_unit] =>
            {
                Ok(Money::from_parts(quantity.parse()?, currency_unit.parse()?))
            },
            _ => Err(MoneyError {
                kind:MoneyErrorKind::Formating,
            })
        }
    }
    
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum CurrencyUnit {
    #[default]
    Lira,
    Dolar,
    Euro,
}

impl Money {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn from_parts(quantity: f64, currency_unit: CurrencyUnit) -> Self {
        Self {
            quantity,
            currency_unit,
        }
    }
}

impl FromStr for CurrencyUnit {
    type Err = MoneyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "lira" | "tl" => Ok(CurrencyUnit::Lira),
            "dolor" | "$" | "usd" => Ok(CurrencyUnit::Dolar),
            "euro" | "€" | "eur" => Ok(CurrencyUnit::Euro),
            _ => Err(MoneyError {
                kind: MoneyErrorKind::CurrencyUnit
            })
        }
    }
}

// Error

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MoneyError {
    kind: MoneyErrorKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum MoneyErrorKind {
    Formating,
    Quantity,
    CurrencyUnit,
}


impl Error for MoneyError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self)
    }
}

impl Display for MoneyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            MoneyErrorKind::Formating => {
                write!(f, "In quantity and currency please.. Example: 42 Lira")
            }
            MoneyErrorKind::Quantity => write!(f, "Quantity parsing error"),
            MoneyErrorKind::CurrencyUnit => write!(f, "Current unit parsing error"),
        }
    }
}

impl From<ParseFloatError> for MoneyError {
    fn from(_: ParseFloatError) -> Self {
        MoneyError {
            kind: MoneyErrorKind::Quantity,
        }
    }
}

impl Termination for Money {
    fn report(self) -> std::process::ExitCode {
        std::process::ExitCode::SUCCESS
    }
}

impl Termination for MoneyError {
    fn report(self) -> std::process::ExitCode {
        std::process::ExitCode::FAILURE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positive_decimals() {
        let result: Result<Money, MoneyError> = "42.1 lira".parse::<Money>();
        assert_eq!(Ok(Money::from_parts(42.1, CurrencyUnit::Lira)), result);
    }
    #[test]
    fn negative_decimals() {
        let result: Result<Money, MoneyError> = "-42.1 lira".parse::<Money>();
        assert_eq!(Ok(Money::from_parts(-42.1, CurrencyUnit::Lira)), result);
    }
 

    #[test]
    fn positive_integer_number() {
        let result: Result<Money, MoneyError> = "42 lira".parse::<Money>();
        assert_eq!(Ok(Money::from_parts(42.0, CurrencyUnit::Lira)), result);
    }
    #[test]
    fn negative_integer_number() {
        let result: Result<Money, MoneyError> = "-42 lira".parse::<Money>();
        assert_eq!(Ok(Money::from_parts(-42.0, CurrencyUnit::Lira)), result);
    }

    #[test]
    #[should_panic]
    fn single_item() {
        "42".parse::<Money>().unwrap();
    }

    #[test]
    #[should_panic]
    fn wrong_quantity() {
        "42a lira".parse::<Money>().unwrap();
    }

    #[test]
    #[should_panic]
    fn multi_item() {
        "42 lira dolar".parse::<Money>().unwrap();
    }
}
