use crate::app::dto::ExpenseDto;
use crate::app::helpers::expense_to_dto;
use crate::app::state::AppData;
use crate::models::expense::{Expense, ExpenseId};
use crate::models::{group::GroupId, user::UserId};
use crate::services::split::Split;

pub fn add_expense(
    data: &mut AppData,
    description: String,
    amount: f64,
    paid_by: UserId,
    group_id: Option<GroupId>,
    splits: Split,
) -> Result<ExpenseDto, String> {
    let id = data.next_expense_id;
    data.next_expense_id += 1;
    let expense = Expense::new(id, &description, amount, paid_by, group_id, splits);
    data.expenses.push(expense);
    Ok(expense_to_dto(data, data.expenses.last().unwrap()))
}

pub fn update_expense(
    data: &mut AppData,
    expense_id: ExpenseId,
    description: Option<String>,
    amount: Option<f64>,
    paid_by: Option<UserId>,
    group_id: Option<Option<GroupId>>,
    splits: Option<Split>,
) -> Result<ExpenseDto, String> {
    {
        let expense = data
            .expenses
            .iter_mut()
            .find(|e| e.id == expense_id)
            .ok_or("Expense not found")?;

        if let Some(desc) = description {
            expense.update_description(&desc);
        }
        if let Some(amt) = amount {
            expense.update_amount(amt);
        }
        if let Some(payer) = paid_by {
            expense.update_paid_by(payer);
        }
        if let Some(gid) = group_id {
            match gid {
                Some(g) => expense.assign_to_group(g),
                None => expense.remove_from_group(),
            }
        }
        if let Some(s) = splits {
            expense.update_splits(s);
        }
    }

    let expense = data
        .expenses
        .iter()
        .find(|e| e.id == expense_id)
        .ok_or("Expense not found")?;

    Ok(expense_to_dto(data, expense))
}

pub fn delete_expense(data: &mut AppData, expense_id: ExpenseId) -> Result<(), String> {
    let index = data
        .expenses
        .iter()
        .position(|e| e.id == expense_id)
        .ok_or("Expense not found")?;
    data.expenses.remove(index);
    Ok(())
}

pub fn list_expenses(data: &AppData) -> Vec<ExpenseDto> {
    data.expenses
        .iter()
        .map(|e| expense_to_dto(data, e))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_expense() {
        let mut data = AppData::default();
        data.users.push(crate::models::user::User::new(
            1,
            "Alice",
            "alice@example.com",
            "hashed_password",
        ));
        let splits = crate::services::split::Split::new_equal(vec![1]).unwrap();
        let dto = add_expense(&mut data, "Dinner".to_string(), 20.0, 1, None, splits).unwrap();
        assert_eq!(dto.description, "Dinner");
        assert_eq!(dto.amount, 20.0);
        assert_eq!(dto.paid_by.to_string(), "1");
        assert_eq!(dto.group_id, None);
        assert_eq!(dto.splits.len(), 1);
        assert_eq!(dto.splits[0].0, "Alice");
        assert_eq!(dto.splits[0].1, 20.0);
    }

    #[test]
    fn test_update_expense() {
        let mut data = AppData::default();
        data.users.push(crate::models::user::User::new(
            1,
            "Alice",
            "alice@example.com",
            "hashed_password",
        ));
        let splits = crate::services::split::Split::new_equal(vec![1]).unwrap();
        let dto = add_expense(&mut data, "Dinner".to_string(), 20.0, 1, None, splits).unwrap();
        assert_eq!(dto.description, "Dinner");
        assert_eq!(dto.amount, 20.0);
        assert_eq!(dto.paid_by.to_string(), "1");
        assert_eq!(dto.group_id, None);
        assert_eq!(dto.splits.len(), 1);
        assert_eq!(dto.splits[0].0, "Alice");
        assert_eq!(dto.splits[0].1, 20.0);
    }

    #[test]
    fn test_delete_expense() {
        let mut data = AppData::default();
        data.users.push(crate::models::user::User::new(
            1,
            "Alice",
            "alice@example.com",
            "hashed_password",
        ));
        let splits = crate::services::split::Split::new_equal(vec![1]).unwrap();
        let dto = add_expense(&mut data, "Dinner".to_string(), 20.0, 1, None, splits).unwrap();
        assert_eq!(dto.description, "Dinner");
        assert_eq!(dto.amount, 20.0);
        assert_eq!(dto.paid_by.to_string(), "1");
        assert_eq!(dto.group_id, None);
        assert_eq!(dto.splits.len(), 1);
        assert_eq!(dto.splits[0].0, "Alice");
        assert_eq!(dto.splits[0].1, 20.0);
    }

    #[test]
    fn test_list_expenses() {
        let mut data = AppData::default();
        data.users.push(crate::models::user::User::new(
            1,
            "Alice",
            "alice@example.com",
            "hashed_password",
        ));
        let splits = crate::services::split::Split::new_equal(vec![1]).unwrap();
        let _dto = add_expense(&mut data, "Dinner".to_string(), 20.0, 1, None, splits).unwrap();
        let expenses = list_expenses(&data);
        assert_eq!(expenses.len(), 1);
        assert_eq!(expenses[0].description, "Dinner");
    }
}
