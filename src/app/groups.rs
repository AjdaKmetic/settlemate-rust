use crate::app::dto::{BalanceDto, DebtDto, ExpenseDto, GroupDto};
use crate::app::helpers::{debt_to_dto, expense_to_dto, name_of};
use crate::app::state::AppData;
use crate::models::{
    expenses::Expense,
    group::{Group, GroupId},
    payment::Payment,
    user::UserId,
};
use crate::services::domain::balance::Balance;
use crate::services::domain::simplify::simplify_debts;

fn expenses_in_group(data: &AppData, group_id: GroupId) -> Vec<Expense> {
    data.expenses
        .iter()
        .filter(|e| e.group_id() == Some(group_id))
        .cloned()
        .collect()
}

fn payments_in_group(data: &AppData, group_id: GroupId) -> Vec<Payment> {
    data.payments
        .iter()
        .filter(|p| p.group_id() == Some(group_id))
        .cloned()
        .collect()
}

pub fn create_group(
    data: &mut AppData,
    name: String,
    member_ids: Vec<UserId>,
) -> Result<GroupDto, String> {
    if name.trim().is_empty() {
        return Err("Group name is required.".into());
    }
    if member_ids.is_empty() {
        return Err("Select at least one member.".into());
    }

    data.next_group_id += 1;
    let id = data.next_group_id;
    let mut group = Group::new(id, &name);
    for &mid in &member_ids {
        group.add_member(mid);
    }
    data.groups.push(group);

    let members: Vec<String> = member_ids.iter().map(|id| name_of(data, *id)).collect();
    Ok(GroupDto {
        id,
        name,
        member_ids,
        members,
        expense_count: 0,
        has_outstanding: false,
        my_balance: 0.0,
    })
}

pub fn list_groups(data: &AppData) -> Vec<GroupDto> {
    data.groups
        .iter()
        .map(|g| {
            let member_ids: Vec<UserId> = g.members().to_vec();
            let members: Vec<String> = member_ids.iter().map(|id| name_of(data, *id)).collect();

            let group_expenses = expenses_in_group(data, g.id);
            let group_payments = payments_in_group(data, g.id);
            let expense_count = group_expenses.len();
            let balance = Balance::balances_with_payments(&group_expenses, &group_payments);

            let my_balance = data
                .current_user_id
                .map(|uid| balance.get(&(uid as UserId)).copied().unwrap_or(0.0))
                .unwrap_or(0.0);

            let has_outstanding = balance.values().any(|v| v.abs() > 0.005);

            GroupDto {
                id: g.id,
                name: g.name().to_string(),
                member_ids,
                members,
                expense_count,
                has_outstanding,
                my_balance,
            }
        })
        .collect()
}

pub fn group_balances(data: &AppData, group_id: GroupId, my_id: UserId) -> Vec<BalanceDto> {
    let group_expenses = expenses_in_group(data, group_id);
    let group_payments = payments_in_group(data, group_id);
    let breakdown = Balance::pairwise_balances(&group_expenses, &group_payments, my_id);

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

pub fn simplify_group_debts(data: &AppData, group_id: GroupId) -> Vec<DebtDto> {
    let group_expenses = expenses_in_group(data, group_id);
    let group_payments = payments_in_group(data, group_id);
    let balance = Balance::balances_with_payments(&group_expenses, &group_payments);
    let debts = simplify_debts(&balance);

    debts.iter().map(|d| debt_to_dto(d, data)).collect()
}

pub fn list_group_expenses(data: &AppData, group_id: GroupId) -> Vec<ExpenseDto> {
    data.expenses
        .iter()
        .filter(|e| e.group_id() == Some(group_id))
        .map(|e| expense_to_dto(data, e))
        .collect()
}

pub fn delete_group(data: &mut AppData, group_id: GroupId) -> Result<(), String> {
    if !data.groups.iter().any(|g| g.id == group_id) {
        return Err("Group not found".to_string());
    }
    data.groups.retain(|g| g.id != group_id);
    data.expenses.retain(|e| e.group_id() != Some(group_id));
    data.payments.retain(|p| p.group_id() != Some(group_id));
    Ok(())
}

pub fn add_member_to_group(
    data: &mut AppData,
    group_id: GroupId,
    user_id: UserId,
) -> Result<(), String> {
    if !data.users.iter().any(|u| u.id == user_id) {
        return Err("User not found".to_string());
    }
    let group = data
        .groups
        .iter_mut()
        .find(|g| g.id == group_id)
        .ok_or("Group not found")?;
    group.add_member(user_id);
    Ok(())
}

pub fn remove_member_from_group(
    data: &mut AppData,
    group_id: GroupId,
    user_id: UserId,
) -> Result<(), String> {
    let group = data
        .groups
        .iter_mut()
        .find(|g| g.id == group_id)
        .ok_or("Group not found")?;
    group.remove_member(user_id);
    Ok(())
}
