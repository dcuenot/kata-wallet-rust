use lib::domain::currency::Currency;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct Money {
    quantity: f32,
    currency: Currency
}

impl Money {
    pub fn new(quantity: f32, currency: Currency) -> Self {
        return Money {
            quantity,
            currency
        }
    }
}

impl Display for Money {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Money ({}, {})", self.quantity, self.currency)
    }
}