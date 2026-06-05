use crate::models::{expenses::ExpenseId, group::GroupId, user::UserId};
use serde::Serialize;

#[derive(Serialize)]
pub struct SplitEntryDto {
    pub user_id: UserId,
    pub amount: f64,
}

#[derive(Serialize)]
pub struct ExpenseDto {
    pub id: ExpenseId,
    pub description: String,
    pub amount: f64,
    pub paid_by: UserId,
    pub group_id: Option<GroupId>,
    pub splits: Vec<(String, f64)>,
    pub created_at: u64,
}

#[derive(Serialize)]
pub struct GroupDto {
    pub id: GroupId,
    pub name: String,
    pub member_ids: Vec<UserId>,
    pub members: Vec<String>,
    pub expense_count: usize,
    pub has_outstanding: bool,
    pub my_balance: f64,
}

#[derive(Serialize)]
pub struct FriendDto {
    pub id: UserId,
    pub name: String,
    pub email: String,
    pub balance: f64,
}

#[derive(Serialize)]
pub struct UserDto {
    pub id: UserId,
    pub name: String,
    pub email: String,
}

#[derive(Serialize)]
pub struct PaymentDto {
    pub id: u64,
    pub from_id: UserId,
    pub from_name: String,
    pub to_id: UserId,
    pub to_name: String,
    pub amount: f64,
    pub group_id: Option<GroupId>,
    pub group_name: Option<String>,
    pub created_at: u64,
}

#[derive(Serialize)]
pub struct BalanceDto {
    pub user_id: UserId,
    pub name: String,
    pub amount: f64,
}

#[derive(Serialize)]
pub struct DebtDto {
    pub from_id: UserId,
    pub from_name: String,
    pub to_id: UserId,
    pub to_name: String,
    pub amount: f64,
}
