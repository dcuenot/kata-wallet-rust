use std::fmt::{Display, Formatter, Result};

// use lib::domain::exchange_rate::ExchangeRate;

#[derive(Debug)]
pub enum Currency {
    EURO,
    DOLLAR,
    POUND,
    YEN,
}

impl Currency {
    pub fn iso(&self) -> &str {
        match *self {
            Currency::EURO => "EUR",
            Currency::DOLLAR => "USD",
            Currency::POUND => "GBP",
            Currency::YEN => "JPY",
        }
    }

    pub fn to_symbol(&self) -> &str {
        match *self {
            Currency::EURO => "€",
            Currency::DOLLAR => "$",
            Currency::POUND => "£",
            Currency::YEN => "¥",
        }
    }

    pub fn from_symbol(symbol: &str) -> Self {
        match symbol {
            "€" => Currency::EURO,
            "$" => Currency::DOLLAR,
            "£" => Currency::POUND,
            "¥" => Currency::YEN,
            _ => unimplemented!()
        }
    }

    pub fn round(&self) -> i8 {
        match *self {
            Currency::EURO => 2,
            Currency::DOLLAR => 2,
            Currency::POUND => 2,
            Currency::YEN => 0,
        }
    }
}

impl Display for Currency {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.iso())
    }
}
//
// pub trait Converter {
//     fn convert_to(&self, currency: Currency) -> ExchangeRate;
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use self::Currency::*;
//
//     #[test]
//     fn test_currency_euro_definition() {
//         assert_eq!(EURO.iso(), "EUR");
//         assert_eq!(EURO.symbol(), "€");
//         assert_eq!(EURO.round(), 2);
//     }
//
//     #[test]
//     fn test_currency_dollar_definition() {
//         assert_eq!(DOLLAR.iso(), "USD");
//         assert_eq!(DOLLAR.symbol(), "$");
//         assert_eq!(DOLLAR.round(), 2);
//     }
// }