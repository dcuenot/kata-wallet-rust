
struct Wallet {
    stocks: String,
}


#[cfg(test)]
mod test {
    use super::*;
    use lib::domain::wallet::test_glue::WalletDSL;


    #[test]
    fn an_empty_wallet_in_euro_should_be_0_euro() {
        // assert_wallet("[] in € = 0,00 €");
        WalletDSL::given_empty_wallet_in_euro().should_be("0.00 €")
    }

}

#[cfg(test)]
mod test_glue {
    use lib::domain::money::Money;
    use lib::domain::currency::Currency;
    use std::error::Error;
    use std::num::ParseFloatError;

    pub struct WalletDSL {
        wallet: String
    }

    impl WalletDSL {
        pub fn given_empty_wallet_in_euro() -> WalletDSL {
            return WalletDSL{ wallet: "[] in €".to_string() }
        }

        pub fn should_be(self, value: &str) {
            println!("wallet: {}", &self.wallet);
            println!("value: {} >> {:?}", value, Self::str_to_money(value));
        }

        fn str_to_money(value: &str) -> Result<Money, ParseFloatError> {
            let slices: Vec<&str> = value.split_ascii_whitespace().collect();
            println!("{}", slices[0]);

            Ok(Money::new(slices[0].parse::<f32>()?, Currency::from_symbol(slices[1])))
        }
    }
}