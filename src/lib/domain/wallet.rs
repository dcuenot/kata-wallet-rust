
struct Wallet {
    stocks: String,
}


#[cfg(test)]
mod test {
    use super::*;
    use lib::domain::wallet::test_glue::WalletDSL;
    use lib::domain::money::Money;
    use lib::domain::currency::Currency;
    use regex::Regex;


    #[test]
    fn an_empty_wallet_in_euro_should_be_0_euro() {
        // assert_wallet("[] in € = 0,00 €");
        WalletDSL::assert("[] with [] in € = 0,00 €")
    }

    #[test]
    fn test_regex() {
        // https://regex101.com/ give the explanation of the regex
        // \[([^]]*)\](?: with \[(.*)\])? in (\S*) = (.*)

        use regex::Regex;
        let re = Regex::new(r"\[(?P<wallet>[^]]*)\](?: with \[(?P<rates>.*)\])? in (?P<target>\S*) = (?P<expected>.*)").unwrap();
        let text = "[] in € = 0.00€";

        if let Some(cap) = re.captures(text) {
            println!("Given a Wallet: [{}]", &cap["wallet"]);
            if let Some(rate) = cap.name("rates") {
                println!("With Exchange rates: [{}]", rate.as_str());
            }
            println!("When we display the wallet in : {}", &cap["target"]);
            println!("The expected result should be : {}", &cap["expected"]);
        }

        // [] in € = 0.00€
        // [] with [] in € = 0.00€
        // [1.00€] in € = 1.00€
        // [1.00€, 1.00$] with [1.00$=1.00€, 1.00$=1.00£] in € = 2,00€
    }

}

#[cfg(test)]
mod test_glue {
    use lib::domain::money::Money;
    use lib::domain::currency::Currency;
    use std::error::Error;
    use std::num::ParseFloatError;

    pub struct WalletDSL {
        wallet: String,
        target_currency: Option<Currency>,
    }

    impl WalletDSL {
        pub fn assert(assertion: &str) -> () {

        }

        pub fn given(wallet_string: &str) -> WalletDSL {
            return WalletDSL{ wallet: wallet_string.to_string(), target_currency: None }
        }

        pub fn when_display_in(self, currency: Currency) {
            // self.target_currency = currency;
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