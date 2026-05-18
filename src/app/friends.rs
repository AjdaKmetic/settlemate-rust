pub fn add_friend(data: &mut AppData, name: String, email: String) -> Result<UserDto, String> {
    let id = data.next_user_id;
    data.next_user_id += 1;
    let user = User::new(id, &name, &email);
    data.users.push(user);
    Ok(UserDto { id, name, email })
}

pub fn list_friends(data: &AppData) -> Vec<UserDto> {
    data.users
        .iter()
        .map(|u| UserDto { id: u.id, name: u.name.clone(), email: u.email.clone() })
        .collect()
}

pub fn list_expenses_for_friend(data: &AppData, friend_id: UserId) -> Vec<ExpenseDto> {
    data.expenses
        .iter()
        .filter(|e| e.paid_by() == friend_id || e.splits.iter().any(|s| s.user_id == friend_id))
        .map(|e| expense_to_dto(data, e))
        .collect()
}

fn friend_breakdown(friend_id: u64, state: State<AppState>) -> Vec<BalanceDto> {
    let data = state.0.lock().unwrap();
    let breakdown = pairwise_balances(&data.expenses, &data.payments, friend_id);
    breakdown.into_iter()
        .filter(|(_, amt)| amt.abs() > 0.005)
        .map(|(id, amount)| BalanceDto { user_id: id, name: name_of(&data, id), amount })
        .collect()
}