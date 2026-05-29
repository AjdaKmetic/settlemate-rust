use std::sync::{Arc, Mutex};

use sea_orm::DatabaseConnection;

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

// da lahko več ljudi naenkrat uporablja podatke
pub type SharedAppData = Arc<Mutex<AppData>>;

pub fn new_shared() -> SharedAppData {
    Arc::new(Mutex::new(AppData::default()))
}

// ko bo baza urejena, odstrani data
#[derive(Clone)]
pub struct AppState {
    pub data: SharedAppData,
    pub db: DatabaseConnection,
}

impl AppState {
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            data: new_shared(),
            db,
        }
    }
}

pub fn seed_demo(data: &SharedAppData) {
    use crate::models::group::Group;
    use crate::models::user::User;

    let mut d = data.lock().unwrap();

    d.users.push(User::new(1, "Ana", "ana@example.com", ""));
    d.users.push(User::new(2, "Juno", "juno@example.com", ""));
    d.users.push(User::new(3, "Ajda", "ajda@example.com", ""));
    d.next_user_id = 4;

    let mut g = Group::new(1, "Amsterdam 2026");
    g.add_member(1);
    g.add_member(2);
    g.add_member(3);
    d.groups.push(g);
    d.next_group_id = 2;
}