use crate::app::state::AppData;
use crate::app::dto::{BalanceDto, ExpenseDto, UserDto};
use crate::app::helpers::{expense_to_dto, name_of};
use crate::models::user::{User, UserId};
use crate::services::balance::Balance;

pub fn add_friend(
    data: &mut AppData,
    name: String,
    email: String,
    password_hash: String,
) -> Result<UserDto, String> {
    data.next_user_id += 1;
    let id = data.next_user_id as UserId;
    let user = User::new(id, &name, &email, &password_hash);
    data.users.push(user);
    Ok(UserDto { id, name, email })
}

pub fn list_friends(data: &AppData) -> Vec<UserDto> {
    data.users
        .iter()
        .filter(|u| data.current_user_id.map_or(true, |cur| u.id != cur))
        .map(|u| UserDto {
            id: u.id,
            name: u.name.clone(),
            email: u.email.clone(),
        })
        .collect()
}

pub fn list_expenses_for_friend(data: &AppData, friend_id: UserId) -> Vec<ExpenseDto> {
    data.expenses
        .iter()
        .filter(|e| {
            e.paid_by() == friend_id
                || e.splits().participants().contains(&friend_id)
        })
        .map(|e| expense_to_dto(data, e))
        .collect()
}

pub fn friend_breakdown(data: &AppData, friend_id: UserId) -> Vec<BalanceDto> {
    let breakdown = Balance::pairwise_balances(&data.expenses, &data.payments, friend_id);
    breakdown
        .iter()
        .filter(|(_, amt)| amt.abs() > 0.005)
        .map(|(id, amount)| BalanceDto {
            user_id: *id,
            name: name_of(data, *id),
            amount: *amount,
        })
        .collect()
}