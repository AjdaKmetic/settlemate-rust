use crate::models::{
    expense::{Expense, ExpenseId},
    group::{Group, GroupId},
    payment::{Payment, PaymentId},
    user::{User, UserId},
};

#[derive(Default)]
pub struct AppData {
    pub users: Vec<User>,
    pub groups: Vec<Group>,
    pub expenses: Vec<Expense>,
    pub payments: Vec<Payment>,
    pub next_user_id: UserId,
    pub next_group_id: GroupId,
    pub next_expense_id: ExpenseId,
    pub next_payment_id: PaymentId,
    pub current_user_id: Option<UserId>,
}
