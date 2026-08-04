use crate::models::{money::Money, user::UserId};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Split {
    Equal(Vec<UserId>),
    Exact(Vec<(UserId, f64)>),
}

impl Split {
    pub fn new_equal(user_ids: Vec<UserId>) -> Result<Self, String> {
        if user_ids.is_empty() {
            return Err("Equal split must have at least one participant".to_string());
        }

        Ok(Split::Equal(user_ids))
    }

    pub fn new_exact(shares: Vec<(UserId, f64)>) -> Result<Self, String> {
        if shares.is_empty() {
            return Err("Exact split must have at least one participant".to_string());
        }

        for (_, amount) in &shares {
            if *amount < 0.0 {
                return Err("Share amounts cannot be negative".to_string());
            }
        }

        Ok(Split::Exact(shares))
    }

    pub fn compute_shares(&self, total_amount: f64) -> Vec<(UserId, f64)> {
        match self {
            Split::Equal(user_ids) => {
                let share = total_amount / user_ids.len() as f64;
                user_ids.iter().map(|&user_id| (user_id, share)).collect()
            }
            Split::Exact(shares) => shares.clone(),
        }
    }

    pub fn participants(&self) -> Vec<UserId> {
        match self {
            Split::Equal(user_ids) => user_ids.clone(),
            Split::Exact(shares) => shares.iter().map(|(user_id, _)| *user_id).collect(),
        }
    }

    pub fn compute_money_shares(
        &self,
        total_amount: Money,
    ) -> Result<Vec<(UserId, Money)>, String> {
        match self {
            Split::Equal(user_ids) => {
                if user_ids.is_empty() {
                    return Err("Equal split must have at least one participant".to_string());
                }

                let amounts = total_amount.split_equal(user_ids.len())?;

                Ok(user_ids.iter().copied().zip(amounts).collect())
            }

            Split::Exact(shares) => {
                if shares.is_empty() {
                    return Err("Exact split must have at least one participant".to_string());
                }

                let mut money_shares = Vec::with_capacity(shares.len());

                for (user_id, amount) in shares {
                    if *amount < 0.0 {
                        return Err("Share amounts cannot be negative".to_string());
                    }

                    let cents = (*amount * 100.0).round() as i64;

                    money_shares.push((*user_id, Money::from_cents(cents)));
                }

                let split_total: i64 = money_shares.iter().map(|(_, money)| money.cents()).sum();

                if split_total != total_amount.cents() {
                    return Err("Exact shares must add up to the total amount".to_string());
                }

                Ok(money_shares)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_equal() {
        let split = Split::new_equal(vec![1, 2, 3]).unwrap();

        let shares = split.compute_shares(90.0);

        assert_eq!(shares, vec![(1, 30.0), (2, 30.0), (3, 30.0)]);
    }

    #[test]
    fn test_split_exact() {
        let split = Split::new_exact(vec![(1, 30.0), (2, 40.0), (3, 20.0)]).unwrap();

        let shares = split.compute_shares(90.0);

        assert_eq!(shares, vec![(1, 30.0), (2, 40.0), (3, 20.0)]);
    }

    #[test]
    fn test_split_participants_equal() {
        let split = Split::new_equal(vec![1, 2, 3]).unwrap();

        let participants = split.participants();

        assert_eq!(participants, vec![1, 2, 3]);
    }

    #[test]
    fn test_split_participants_exact() {
        let split = Split::new_exact(vec![(1, 10.0), (2, 20.0), (3, 30.0)]).unwrap();

        let participants = split.participants();

        assert_eq!(participants, vec![1, 2, 3]);
    }

    #[test]
    fn test_split_equal_two_users() {
        let split = Split::new_equal(vec![1, 2]).unwrap();

        let shares = split.compute_shares(50.0);

        assert_eq!(shares, vec![(1, 25.0), (2, 25.0)]);
    }

    #[test]
    fn test_split_equal_one_user() {
        let split = Split::new_equal(vec![1]).unwrap();

        let shares = split.compute_shares(50.0);

        assert_eq!(shares, vec![(1, 50.0)]);
    }

    #[test]
    fn test_split_exact_not_dependent_on_total_amount() {
        let split = Split::new_exact(vec![(1, 10.0), (2, 20.0)]).unwrap();

        let shares = split.compute_shares(999.0);

        assert_eq!(shares, vec![(1, 10.0), (2, 20.0)]);
    }

    #[test]
    fn test_new_equal_empty_returns_error() {
        let split = Split::new_equal(vec![]);

        assert!(split.is_err());
        assert_eq!(
            split.unwrap_err(),
            "Equal split must have at least one participant"
        );
    }

    #[test]
    fn test_new_exact_empty_returns_error() {
        let split = Split::new_exact(vec![]);

        assert!(split.is_err());
        assert_eq!(
            split.unwrap_err(),
            "Exact split must have at least one participant"
        );
    }

    #[test]
    fn test_new_exact_negative_amount_returns_error() {
        let split = Split::new_exact(vec![(1, -10.0), (2, 20.0)]);

        assert!(split.is_err());
        assert_eq!(split.unwrap_err(), "Share amounts cannot be negative");
    }

    #[test]
    fn test_equal_money_split_preserves_every_cent() {
        let split = Split::new_equal(vec![1, 2, 3]).unwrap();

        let shares = split.compute_money_shares(Money::from_cents(1000)).unwrap();

        assert_eq!(
            shares,
            vec![
                (1, Money::from_cents(334)),
                (2, Money::from_cents(333)),
                (3, Money::from_cents(333)),
            ]
        );

        let total: i64 = shares.iter().map(|(_, amount)| amount.cents()).sum();

        assert_eq!(total, 1000);
    }

    #[test]
    fn test_exact_money_split_accepts_correct_total() {
        let split = Split::new_exact(vec![(1, 10.25), (2, 20.50), (3, 19.25)]).unwrap();

        let shares = split.compute_money_shares(Money::from_cents(5000)).unwrap();

        assert_eq!(
            shares,
            vec![
                (1, Money::from_cents(1025)),
                (2, Money::from_cents(2050)),
                (3, Money::from_cents(1925)),
            ]
        );
    }

    #[test]
    fn test_exact_money_split_rejects_wrong_total() {
        let split = Split::new_exact(vec![(1, 10.00), (2, 15.00)]).unwrap();

        let result = split.compute_money_shares(Money::from_cents(3000));

        assert_eq!(
            result.unwrap_err(),
            "Exact shares must add up to the total amount"
        );
    }
}
