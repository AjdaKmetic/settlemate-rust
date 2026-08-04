use ::std::ops::{Add, AddAssign, Neg, Sub};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Money {
    cents: i64,
}

impl Money {
    pub const ZERO: Self = Self { cents: 0 };

    pub const fn from_cents(cents: i64) -> Self {
        Self { cents }
    }

    pub const fn cents(&self) -> i64 {
        self.cents
    }

    pub const fn is_zero(&self) -> bool {
        self.cents == 0
    }

    pub const fn is_positive(&self) -> bool {
        self.cents > 0
    }

    pub const fn is_negative(&self) -> bool {
        self.cents < 0
    }

    pub fn abs(&self) -> Self {
        Self {
            cents: self.cents.abs(),
        }
    }

    pub fn split_equal(self, participant_count: usize) -> Result<Vec<Self>, String> {
        if participant_count == 0 {
            return Err("Cannot split among zero participants".to_string());
        }

        if self.is_negative() {
            return Err("Cannot split a negative amount".to_string());
        }

        let count = participant_count as i64;
        let base_amount = self.cents / count;
        let remainder = self.cents % count;

        let shares = (0..participant_count)
            .map(|i| {
                if i < remainder as usize {
                    Self::from_cents(base_amount + 1)
                } else {
                    Self::from_cents(base_amount)
                }
            })
            .collect();

        Ok(shares)
    }
}

impl Add for Money {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            cents: self.cents + other.cents,
        }
    }
}

impl AddAssign for Money {
    fn add_assign(&mut self, other: Self) {
        self.cents += other.cents;
    }
}

impl Sub for Money {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            cents: self.cents - other.cents,
        }
    }
}

impl Neg for Money {
    type Output = Self;

    fn neg(self) -> Self {
        Self { cents: -self.cents }
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let absolute_cents = self.cents.abs();
        let euros = absolute_cents / 100;
        let cents = absolute_cents % 100;
        let sign = if self.cents < 0 { "-" } else { "" };
        write!(f, "{}{}.{:02} €", sign, euros, cents)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_cents() {
        let money = Money::from_cents(150);
        assert_eq!(money.cents(), 150);
    }

    #[test]
    fn test_is_zero() {
        let money = Money::from_cents(0);
        assert!(money.is_zero());
    }

    #[test]
    fn test_is_positive() {
        let money = Money::from_cents(100);
        assert!(money.is_positive());
    }

    #[test]
    fn test_is_negative() {
        let money = Money::from_cents(-100);
        assert!(money.is_negative());
    }

    #[test]
    fn test_abs() {
        let money = Money::from_cents(-150);
        assert_eq!(money.abs().cents(), 150);
    }

    #[test]
    fn test_add() {
        let money1 = Money::from_cents(100);
        let money2 = Money::from_cents(200);
        let result = money1 + money2;
        assert_eq!(result.cents(), 300);
    }

    #[test]
    fn test_sub() {
        let money1 = Money::from_cents(300);
        let money2 = Money::from_cents(100);
        let result = money1 - money2;
        assert_eq!(result.cents(), 200);
    }

    #[test]
    fn test_neg() {
        let money = Money::from_cents(150);
        let result = -money;
        assert_eq!(result.cents(), -150);
    }

    #[test]
    fn test_display_formats_positive_and_negative_amounts() {
        let positive_money = Money::from_cents(12345);
        let negative_money = Money::from_cents(-6789);

        assert_eq!(format!("{}", positive_money), "123.45 €");
        assert_eq!(format!("{}", negative_money), "-67.89 €");
    }

    #[test]
    fn test_split_equal_preserves_every_cent() {
        let amount = Money::from_cents(1000);

        let shares = amount.split_equal(3).unwrap();

        assert_eq!(
            shares,
            vec![
                Money::from_cents(334),
                Money::from_cents(333),
                Money::from_cents(333),
            ]
        );

        let total: i64 = shares.iter().map(|share| share.cents()).sum();

        assert_eq!(total, 1000);
    }
}
