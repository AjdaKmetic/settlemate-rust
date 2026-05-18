#[derive(Default)]
struct AppData {
    users: Vec<User>,
    groups: Vec<Group>,
    expenses: Vec<Expense>,
    payments: Vec<Payment>,
    next_user_id: u64,
    next_group_id: u64,
    next_expense_id: u64,
    next_payment_id: u64,
    current_user_id: Option<u64>,
}