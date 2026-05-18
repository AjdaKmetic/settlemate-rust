fn create_group(name: String, member_ids: Vec<u64>, state: State<AppState>) -> Result<GroupDto, String> {
    if name.trim().is_empty() { return Err("Group name is required.".into()); }
    if member_ids.is_empty() { return Err("Select at least one member.".into()); }
    let mut data = state.0.lock().unwrap();
    data.next_group_id += 1;
    let id = data.next_group_id;
    let mut group = Group::new(id, &name);
    for &mid in &member_ids {
        group.add_member(mid);
    }
    data.groups.push(group);
    let members: Vec<String> = member_ids.iter().map(|id| name_of(&data, *id)).collect();
    Ok(GroupDto { id, name, member_ids, members, expense_count: 0, has_outstanding: false, my_balance: 0.0})
}

pub fn list_groups(data: &AppData) -> Vec<GroupDto> {
    data.groups.iter().map(|g| {
        let member_ids = g.members.clone();
        let members: Vec<String> = member_ids.iter().map(|id| name_of(data, *id)).collect();
        GroupDto {
            id: g.id,
            name: g.name.clone(),
            member_ids,
            members,
            expense_count: data.expenses.iter().filter(|e| e.group_id() == Some(g.id)).count(),
            has_outstanding: data.expenses.iter().any(|e| e.group_id() == Some(g.id) && e.splits.iter().any(|s| s.amount > 0.005)),
            my_balance: calculate_group_balance(&data.expenses, &data.payments, g.id, data.current_user_id),
        }
    }).collect()
}

pub fn group_balances(group_id: u64, state: State<AppState>) -> Vec<BalanceDto> {
    let data = state.0.lock().unwrap();
    let breakdown = group_pairwise_balances(&data.expenses, &data.payments, group_id);
    breakdown.into_iter()
        .filter(|(_, amt)| amt.abs() > 0.005)
        .map(|(id, amount)| BalanceDto { user_id: id, name: name_of(&data, id), amount })
        .collect()
}

pub fn simplify_group_debts(group_id: u64, state: State<AppState>) -> Vec<PaymentDto> {
    let data = state.0.lock().unwrap();
    let payments = simplify_debts(group_id, &data.expenses, &data.payments);
    payments.into_iter().map(|p| payment_to_dto(&p, &data)).collect()
}

pub fn list_group_expenses(group_id: u64, state: State<AppState>) -> Vec<ExpenseDto> {
    let data = state.0.lock().unwrap();
    data.expenses.iter()
        .filter(|e| e.group_id() == Some(group_id))
        .map(|e| expense_to_dto(&data, e))
        .collect()
}

fn delete_group(group_id: u64, state: State<AppState>) -> Result<(), String> {
    let mut data = state.0.lock().unwrap();
    if !data.groups.iter().any(|g| g.id == group_id) {
        return Err("Group not found".to_string());
    }
    data.groups.retain(|g| g.id != group_id);
    data.expenses.retain(|e| e.group_id() != Some(group_id));
    data.payments.retain(|p| p.group_id() != Some(group_id));
    Ok(())
}

pub fn add_member_to_group(group_id: u64, user_id: u64, state: State<AppState>) -> Result<(), String> {
    let mut data = state.0.lock().unwrap();
    let group = data.groups.iter_mut().find(|g| g.id == group_id).ok_or("Group not found")?;
    if !data.users.iter().any(|u| u.id == user_id) {
        return Err("User not found".to_string());
    }
    group.add_member(user_id);
    Ok(())
}

pub fn remove_member_from_group(group_id: u64, user_id: u64, state: State<AppState>) -> Result<(), String> {
    let mut data = state.0.lock().unwrap();
    let group = data.groups.iter_mut().find(|g| g.id == group_id).ok_or("Group not found")?;
    group.remove_member(user_id);
    Ok(())
}