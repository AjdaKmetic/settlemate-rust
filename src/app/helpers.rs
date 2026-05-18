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
    let paid_by_name = name_of(data, expense.paid_by());
    let group_name = expense.group_id().map(|gid| group_name_of(data, gid));
    ExpenseDto {
        id: expense.id,
        description: expense.description().to_string(),
        amount: expense.amount(),
        paid_by: expense.paid_by(),
        paid_by_name,
        group_id: expense.group_id(),
        group_name,
        splits: expense.splits.clone(),
        created_at: expense.created_at(),
    }
}

fn payment_to_dto(p: &Payment, data: &AppData) -> PaymentDto {
    PaymentDto {
        id: p.id,
        from_id: p.from_id(),
        from_name: name_of(data, p.from_id()),
        to_id: p.to_id(),
        to_name: name_of(data, p.to_id()),
        amount: p.amount(),
        group_id: p.group_id(),
        group_name: p.group_id().and_then(|id| group_name_of(data, id)),
        created_at: p.created_at(),
    }
}