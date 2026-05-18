use crate::app::dto::{DebtDto, ExpenseDto, PaymentDto};
use crate::app::state::AppData;
use crate::models::{debt::Debt, expense::Expense, group::GroupId, payment::Payment, user::UserId};

pub fn name_of(data: &AppData, user_id: UserId) -> String {
    data.users
        .iter()
        .find(|u| u.id == user_id)
        .map(|u| u.name.clone())
        .unwrap_or_else(|| "Unknown".to_string())
}

pub fn group_name_of(data: &AppData, group_id: GroupId) -> String {
    data.groups
        .iter()
        .find(|g| g.id == group_id)
        .map(|g| g.name.clone())
        .unwrap_or_else(|| "Unknown".to_string())
}

pub fn expense_to_dto(data: &AppData, expense: &Expense) -> ExpenseDto {
    let splits = expense
        .splits()
        .compute_shares(expense.amount())
        .into_iter()
        .map(|(user_id, amount)| (name_of(data, user_id), amount))
        .collect();

    ExpenseDto {
        id: expense.id,
        description: expense.description().to_string(),
        amount: expense.amount(),
        paid_by: expense.paid_by(),
        group_id: expense.group_id(),
        splits,
        created_at: expense.created_at(),
    }
}

pub fn payment_to_dto(p: &Payment, data: &AppData) -> PaymentDto {
    PaymentDto {
        id: p.id,
        from_id: p.from_id(),
        from_name: name_of(data, p.from_id()),
        to_id: p.to_id(),
        to_name: name_of(data, p.to_id()),
        amount: p.amount(),
        group_id: p.group_id(),
        group_name: p.group_id().map(|id| group_name_of(data, id)),
        created_at: p.created_at(),
    }
}

pub fn debt_to_dto(debt: &Debt, data: &AppData) -> DebtDto {
    DebtDto {
        from_id: debt.from(),
        from_name: name_of(data, debt.from()),
        to_id: debt.to(),
        to_name: name_of(data, debt.to()),
        amount: debt.amount(),
    }
}
