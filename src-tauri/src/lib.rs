use std::sync::Mutex;
use tauri::State;

use settlemate_rust::app::{
    balances, current_user,
    dto::{BalanceDto, DebtDto, ExpenseDto, GroupDto, PaymentDto, UserDto},
    expenses, friends, groups, payments,
    state::AppData,
};
use settlemate_rust::models::{expense::ExpenseId, group::GroupId, user::UserId};
use settlemate_rust::services::split::Split;

pub struct AppState(pub Mutex<AppData>);

// FRIENDS
#[tauri::command]
fn add_friend(name: String, state: State<AppState>) -> Result<UserDto, String> {
    let mut data = state.0.lock().unwrap();
    friends::add_friend(&mut data, name, String::new(), String::new())
}

#[tauri::command]
fn list_friends(state: State<AppState>) -> Vec<UserDto> {
    let data = state.0.lock().unwrap();
    friends::list_friends(&data)
}

#[tauri::command]
fn list_expenses_for_friend(friend_id: UserId, state: State<AppState>) -> Vec<ExpenseDto> {
    let data = state.0.lock().unwrap();
    friends::list_expenses_for_friend(&data, friend_id)
}

#[tauri::command]
fn friend_breakdown(friend_id: UserId, state: State<AppState>) -> Vec<BalanceDto> {
    let data = state.0.lock().unwrap();
    friends::friend_breakdown(&data, friend_id)
}

// GROUPS

#[tauri::command]
fn create_group(
    name: String,
    member_ids: Vec<UserId>,
    state: State<AppState>,
) -> Result<GroupDto, String> {
    let mut data = state.0.lock().unwrap();
    groups::create_group(&mut data, name, member_ids)
}

#[tauri::command]
fn list_groups(state: State<AppState>) -> Vec<GroupDto> {
    let data = state.0.lock().unwrap();
    groups::list_groups(&data)
}

#[tauri::command]
fn group_balances(group_id: GroupId, state: State<AppState>) -> Result<Vec<BalanceDto>, String> {
    let data = state.0.lock().unwrap();
    let my_id = data
        .current_user_id
        .ok_or("No current user set".to_string())?;
    Ok(groups::group_balances(&data, group_id, my_id))
}

#[tauri::command]
fn simplify_group(group_id: GroupId, state: State<AppState>) -> Vec<DebtDto> {
    let data = state.0.lock().unwrap();
    groups::simplify_group_debts(&data, group_id)
}

#[tauri::command]
fn list_group_expenses(group_id: GroupId, state: State<AppState>) -> Vec<ExpenseDto> {
    let data = state.0.lock().unwrap();
    groups::list_group_expenses(&data, group_id)
}

#[tauri::command]
fn delete_group(group_id: GroupId, state: State<AppState>) -> Result<(), String> {
    let mut data = state.0.lock().unwrap();
    groups::delete_group(&mut data, group_id)
}

#[tauri::command]
fn add_member_to_group(
    group_id: GroupId,
    user_id: UserId,
    state: State<AppState>,
) -> Result<(), String> {
    let mut data = state.0.lock().unwrap();
    groups::add_member_to_group(&mut data, group_id, user_id)
}

#[tauri::command]
fn remove_member_from_group(
    group_id: GroupId,
    user_id: UserId,
    state: State<AppState>,
) -> Result<(), String> {
    let mut data = state.0.lock().unwrap();
    groups::remove_member_from_group(&mut data, group_id, user_id)
}

// EXPENSES

#[tauri::command]
fn add_expense(
    description: String,
    amount: f64,
    paid_by: UserId,
    group_id: Option<GroupId>,
    splits: Split,
    state: State<AppState>,
) -> Result<ExpenseDto, String> {
    let mut data = state.0.lock().unwrap();
    expenses::add_expense(&mut data, description, amount, paid_by, group_id, splits)
}

#[tauri::command]
fn update_expense(
    expense_id: ExpenseId,
    description: Option<String>,
    amount: Option<f64>,
    paid_by: Option<UserId>,
    group_id: Option<Option<GroupId>>,
    splits: Option<Split>,
    state: State<AppState>,
) -> Result<ExpenseDto, String> {
    let mut data = state.0.lock().unwrap();
    expenses::update_expense(
        &mut data,
        expense_id,
        description,
        amount,
        paid_by,
        group_id,
        splits,
    )
}

#[tauri::command]
fn delete_expense(expense_id: ExpenseId, state: State<AppState>) -> Result<(), String> {
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
    from_id: UserId,
    to_id: UserId,
    amount: f64,
    group_id: Option<GroupId>,
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
fn list_payments_for_friend(friend_id: UserId, state: State<AppState>) -> Vec<PaymentDto> {
    let data = state.0.lock().unwrap();
    payments::list_payments_for_friend(&data, friend_id)
}

#[tauri::command]
fn list_payments_for_group(group_id: GroupId, state: State<AppState>) -> Vec<PaymentDto> {
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
fn simplify(state: State<AppState>) -> Vec<DebtDto> {
    let data = state.0.lock().unwrap();
    balances::simplify_balances(&data)
}

// CURRENT USER

#[tauri::command]
fn set_current_user(user_id: UserId, state: State<AppState>) -> Result<(), String> {
    let mut data = state.0.lock().unwrap();
    current_user::set_current_user(&mut data, user_id)
}

#[tauri::command]
fn clear_current_user(state: State<AppState>) {
    let mut data = state.0.lock().unwrap();
    current_user::clear_current_user(&mut data);
}

#[tauri::command]
fn get_current_user(state: State<AppState>) -> Option<UserDto> {
    let data = state.0.lock().unwrap();
    current_user::get_current_user(&data)
}

//  ENTRY POINT

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState(Mutex::new(AppData::default())))
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Friends
            add_friend,
            list_friends,
            list_expenses_for_friend,
            friend_breakdown,
            // Groups
            create_group,
            list_groups,
            group_balances,
            simplify_group,
            list_group_expenses,
            delete_group,
            add_member_to_group,
            remove_member_from_group,
            // Expenses
            add_expense,
            update_expense,
            delete_expense,
            list_expenses,
            // Payments
            record_payment,
            delete_payment,
            list_payments,
            list_payments_for_friend,
            list_payments_for_group,
            // Balances
            get_balances,
            simplify,
            // Current user
            get_current_user,
            set_current_user,
            clear_current_user,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
