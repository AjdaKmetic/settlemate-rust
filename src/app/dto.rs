use serde::Serialize;

 #[derive(Serialize)]
pub struct GroupDto {
     id: u32,
     name: String,
     member_ids: Vec<u32>,
     members: Vec<String>, 
     expense_count: usize,
     has_outstanding: bool,
     my_balance: f64,
 }

    #[derive(Serialize)]
 pub struct FriendDto {
     id: u32,
     name: String,
     email: String,
     balance: f64,
 }

    #[derive(Serialize)]
 pub struct ExpenseDto {
     id: u32,
     description: String,
     amount: f64,
     paid_by: String,
     group_id: Option<u32>,
     splits: Vec<(String, f64)>,
     created_at: u64,
 }

    #[derive(Serialize)]
 pub struct PaymentDto {
     id: u32,
     from_id: u32,
     from_name: String,
     to_id: u32,
     to_name: String,
     amount: f64,
     group_id: Option<u32>,
     group_name: Option<String>,
     created_at: u64,
 }

    #[derive(Serialize)]
 pub struct BalanceDto {
     total_balance: f64,
     group_balances: Vec<(String, f64)>,
 }

    #[derive(Serialize)]
 pub struct SettlmentDto {
     id: u32,
     with_user_id: u32,
     with_user_name: String,
     amount: f64,
     created_at: u64,
 }