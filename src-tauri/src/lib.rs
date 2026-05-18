use std::sync::Mutex;
use tauri::State;

use settlemate_rust::app::{
    state::AppData,
    dto::*,
    friends,
    groups,
    expenses,
    payments,
    balances,
    current_user,
};

struct AppState(Mutex<AppData>);

// FRIENDS

#[tauri::command]
fn add_friend(name: String, state: State<AppState>) -> FriendDto {
    let mut data = state.0.lock().unwrap();
    friends::add_friend(&mut data, name)
}

#[tauri::command]
fn list_friends(state: State<AppState>) -> Vec<FriendDto> {
    let data = state.0.lock().unwrap();
    friends::list_friends(&data)
}
#[tauri::command]
fn list_expenses_for_friend(friend_id: u64, state: State<AppState>) -> Vec<ExpenseDto> {
    let data = state.0.lock().unwrap();
    friends::list_expenses_for_friend(&data, friend_id)
}

#[tauri::command]
fn friend_breakdown(friend_id: u64, state: State<AppState>) -> Vec<BalanceDto> {
    let data = state.0.lock().unwrap();
    friends::friend_breakdown(&data, friend_id)
}

#[tauri::command]
fn remove_friend(id: u64, state: State<AppState>) -> Result<(), String> {
    let mut data = state.0.lock().unwrap();
    friends::remove_friend(&mut data, id)
}

// GROUPS

#[tauri::command]
fn create_group(name: String, state: State<AppState>) -> GroupDto {
    let mut data = state.0.lock().unwrap();
    groups::create_group(&mut data, name)
}

#[tauri::command]
fn list_groups(state: State<AppState>) -> Vec<GroupDto> {
    let data = state.0.lock().unwrap();
    groups::list_groups(&data)
}

#[tauri::command]
fn group_balances(group_id: u64, state: State<AppState>) -> Vec<BalanceDto> {
    let data = state.0.lock().unwrap();
    groups::group_balances(&data, group_id)
}

#[tauri::command]
fn simplify_group_balances(group_id: u64, state: State<AppState>) -> Vec<DebtDto> {
    let data = state.0.lock().unwrap();
    groups::simplify_group_balances(&data, group_id)
}

#[tauri::command]
fn add_member_to_group(group_id: u64, user_id: u64, state: State<AppState>) -> Result<(), String> {
    let mut data = state.0.lock().unwrap();
    groups::add_member_to_group(&mut data, group_id, user_id)
}

#[tauri::command]
fn remove_member_from_group(group_id: u64, user_id: u64, state: State<AppState>) -> Result<(), String> {
    let mut data = state.0.lock().unwrap();
    groups::remove_member_from_group(&mut data, group_id, user_id)
}

#[tauri::command]
fn delete_group(group_id: u64, state: State<AppState>) -> Result<(), String> {
    let mut data = state.0.lock().unwrap();
    groups::delete_group(&mut data, group_id)
}

#[tauri::command]
fn list_group_members(group_id: u64, state: State<AppState>) -> Vec<UserDto> {
    let data = state.0.lock().unwrap();
    groups::list_group_members(&data, group_id)
}

#[tauri::command]
fn list_group_expenses(group_id: u64, state: State<AppState>) -> Vec<ExpenseDto> {
    let data = state.0.lock().unwrap();
    groups::list_group_expenses(&data, group_id)
}

// EXPENSES

#[tauri::command]
fn add_expense(  
    description: String,
    amount: f64,
    paid_by: u64,
    group_id: Option<u64>,
    splits: Vec<(u64, f64)>,
    state: State<AppState>,
) -> Result<ExpenseDto, String> {
    let mut data = state.0.lock().unwrap();
    expenses::add_expense(&mut data, description, amount, paid_by, group_id, splits)
}

#[tauri::command]
fn update_expense(
    expense_id: u64,
    description: Option<String>,
    amount: Option<f64>,
    paid_by: Option<u64>,
    group_id: Option<Option<u64>>,
    splits: Option<Vec<(u64, f64)>>,
    state: State<AppState>,
) -> Result<ExpenseDto, String> {
    let mut data = state.0.lock().unwrap();
    expenses::update_expense(&mut data, expense_id, description, amount, paid_by, group_id, splits)
}

#[tauri::command]
fn delete_expense(expense_id: u64, state: State<AppState>) -> Result<(), String> {
    let mut data = state.0.lock().unwrap();
    expenses::delete_expense(&mut data, expense_id)
}

#[tauri::command]
fn list_expenses(state: State<AppState>) -> Vec<ExpenseDto> {
    let data = state.0.lock().unwrap();
    expenses::list_expenses(&data)
}

// PAYMENTS

#[tauri::command]
fn record_payment(
    from_id: u64,
    to_id: u64,
    amount: f64,
    group_id: Option<u64>,
    state: State<AppState>,
) -> Result<PaymentDto, String> {
    let mut data = state.0.lock().unwrap();
    payments::record_payment(&mut data, from_id, to_id, amount, group_id)
}

#[tauri::command]
fn delete_payment(payment_id: u64, state: State<AppState>) -> Result<(), String> {
    let mut data = state.0.lock().unwrap();
    payments::delete_payment(&mut data, payment_id)
}

#[tauri::command]
fn list_payments(state: State<AppState>) -> Vec<PaymentDto> {
    let data = state.0.lock().unwrap();
    payments::list_payments(&data)
}

#[tauri::command]
fn list_payments_for_friend(friend_id: u64, state: State<AppState>) -> Vec<PaymentDto> {
    let data = state.0.lock().unwrap();
    payments::list_payments_for_friend(&data, friend_id)
}

#[tauri::command]
fn list_payments_for_group(group_id: u64, state: State<AppState>) -> Vec<PaymentDto> {
    let data = state.0.lock().unwrap();
    payments::list_payments_for_group(&data, group_id)
}

// BALANCES

#[tauri::command]
fn get_balances(state: State<AppState>) -> Vec<BalanceDto> {
    let data = state.0.lock().unwrap();
    balances::get_balances(&data)
}

#[tauri::command]
fn get_balance_with_friend(friend_id: u64, state: State<AppState>) -> Vec<BalanceDto> {
    let data = state.0.lock().unwrap();
    balances::get_balance_with_friend(&data, friend_id)
}

#[tauri::command]
fn get_balance_with_group(group_id: u64, state: State<AppState>) -> Vec<BalanceDto> {
    let data = state.0.lock().unwrap();
    balances::get_balance_with_group(&data, group_id)
}

#[tauri::command]
fn simplify_balances(state: State<AppState>) -> Vec<DebtDto> {
    let data = state.0.lock().unwrap();
    balances::simplify_balances(&data)
}

#[tauri::command]
fn simplify_balances_with_friend(friend_id: u64, state: State<AppState>) -> Vec<DebtDto> {
    let data = state.0.lock().unwrap();
    balances::simplify_balances_with_friend(&data, friend_id)
}

#[tauri::command]
fn simplify_balances_with_group(group_id: u64, state: State<AppState>) -> Vec<DebtDto> {
    let data = state.0.lock().unwrap();
    balances::simplify_balances_with_group(&data, group_id)
}

// CURRENT USER

#[tauri::command]
fn login(email: String, password: String, state: State<AppState>) -> Result<UserDto, String> {
    let mut data = state.0.lock().unwrap();
    current_user::login(&mut data, email, password)
}

#[tauri::command]
fn logout(state: State<AppState>) {
    let mut data = state.0.lock().unwrap();
    current_user::logout(&mut data);
}

#[tauri::command]
fn get_current_user(state: State<AppState>) -> Option<UserDto> {
    let data = state.0.lock().unwrap();
    current_user::get_current_user(&data)
}

#[tauri::command]
fn set_current_user(user_id: Option<u64>, state: State<AppState>) -> Result<(), String> {
    let mut data = state.0.lock().unwrap();
    current_user::set_current_user(&mut data, user_id)
}

// ENTRY POINT

#[cfg_aftr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState(Mutex::new(AppData::default())))
        .invoke_handler(tauri::generate_handler![
            add_friend,
            list_friends,
            list_expenses_for_friend,
            friend_breakdown,
            remove_friend,
            create_group,
            list_groups,
            group_balances,
            simplify_group_balances,
            add_member_to_group,
            remove_member_from_group,
            delete_group,
            list_group_members,
            list_group_expenses,
            add_expense,
            update_expense,
            delete_expense,
            list_expenses,
            record_payment,
            delete_payment,
            list_payments,
            list_payments_for_friend,
            list_payments_for_group,
            get_balances,
            get_balance_with_friend,
            get_balance_with_group,
            simplify_balances,
            simplify_balances_with_friend,
            simplify_balances_with_group,
            login,
            logout,
            get_current_user,
            set_current_user
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

