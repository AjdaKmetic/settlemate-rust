use crate::app::state::AppData;
use crate::app::dto::PaymentDto;
use crate::app::helpers;
use crate::models::{
    payment::Payment,
    user::UserId,
    group::GroupId,
};

pub fn record_payment(
    data: &mut AppData,
    from_id: UserId,
    to_id: UserId,
    amount: f64,
    group_id: Option<GroupId>,
) -> Result<PaymentDto, String> {
    let id = data.next_payment_id;
    data.next_payment_id += 1;
    let payment = Payment::new(id, from_id, to_id, amount, group_id)?;
    data.payments.push(payment);

    let last = data.payments.last().unwrap().clone();
    Ok(helpers::payment_to_dto(&last, data))
}

pub fn delete_payment(data: &mut AppData, payment_id: u64) -> Result<(), String> {
    let index = data.payments
        .iter()
        .position(|p| p.id == payment_id)
        .ok_or("Payment not found")?;
    data.payments.remove(index);
    Ok(())
}

pub fn list_payments(data: &AppData) -> Vec<PaymentDto> {
    data.payments
        .iter()
        .map(|p| helpers::payment_to_dto(p, data))
        .collect()
}

pub fn list_payments_for_friend(data: &AppData, friend_id: UserId) -> Vec<PaymentDto> {
    data.payments
        .iter()
        .filter(|p| p.from_id() == friend_id || p.to_id() == friend_id)
        .map(|p| helpers::payment_to_dto(p, data))
        .collect()
}

pub fn list_payments_for_group(data: &AppData, group_id: GroupId) -> Vec<PaymentDto> {
    data.payments
        .iter()
        .filter(|p| p.group_id() == Some(group_id))
        .map(|p| helpers::payment_to_dto(p, data))
        .collect()
}