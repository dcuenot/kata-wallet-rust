// use lib::domain::currency::Currency;
// use lib::domain::currency::Converter;
// use lib::domain::exchange_rate::ExchangeRate;
//
// impl Converter for Currency {
//     fn convert_to(&self, _currency: Currency) -> ExchangeRate {
//         unimplemented!()
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use lib::domain::currency::Currency::*;
//     use lib::domain::currency::{Converter, Currency};
//     use lib::domain::exchange_rate::ExchangeRate;
//
//     impl Converter for Currency {
//         fn convert_to(&self, _currency: Currency) -> ExchangeRate {
//             ExchangeRate::of(0.1)
//         }
//     }
//
//     #[test]
//     fn test_converter() {
//         assert_eq!(EURO.convert_to(YEN), ExchangeRate::of(0.1));
//         assert_eq!(EURO.convert_to(POUND), ExchangeRate::of(0.1));
//     }
// }