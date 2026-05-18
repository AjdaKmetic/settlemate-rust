use crate::app::dto::UserDto;
use crate::app::state::AppData;
use crate::models::user::UserId;

pub fn get_current_user(data: &AppData) -> Option<UserDto> {
    let my_id = data.current_user_id?;

    data.users
        .iter()
        .find(|u| u.id == my_id)
        .map(|u| UserDto {
            id: u.id,
            name: u.name.clone(),
            email: u.email.clone(),
        })
}

pub fn set_current_user(data: &mut AppData, user_id: UserId) -> Result<(), String> {
    if !data.users.iter().any(|u| u.id == user_id) {
        return Err("User not found".to_string());
    }

    data.current_user_id = Some(user_id);
    Ok(())
}

pub fn clear_current_user(data: &mut AppData) {
    data.current_user_id = None;
}