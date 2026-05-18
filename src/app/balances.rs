use crate::app::dto::{BalanceDto, DebtDto};
use crate::app::helpers::{debt_to_dto, name_of};
use crate::app::state::AppData;
use crate::services::balance::Balance;
use crate::services::simplify::simplify_debts;

pub fn get_balances(data: &AppData) -> Vec<BalanceDto> {
    let balance = Balance::balances_with_payments(&data.expenses, &data.payments);
    balance
        .iter()
        .map(|(id, amount)| BalanceDto {
            user_id: *id,
            name: name_of(data, *id),
            amount: *amount,
        })
        .collect()
}

pub fn simplify_balances(data: &AppData) -> Vec<DebtDto> {
    let balance = Balance::balances_with_payments(&data.expenses, &data.payments);
    let debts = simplify_debts(&balance);
    debts.iter().map(|d| debt_to_dto(d, data)).collect()
}
