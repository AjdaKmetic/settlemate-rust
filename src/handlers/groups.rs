use askama::Template;
use axum::{
    extract::{Form, Path, State},
    http::StatusCode,
    response::{Html, IntoResponse},
};
use serde::Deserialize;

use crate::app::state::AppState;
use crate::entities::{groups, users};
use crate::services::db::group_service::{
    add_member_to_group, create_group, find_group_by_id, get_all_groups,
};
use crate::services::db::user_service::get_all_users;

#[derive(Template)]
#[template(path = "groups.html")]
struct GroupsTemplate {
    groups: Vec<groups::Model>,
}

#[derive(Template)]
#[template(path = "group_item.html")]
struct GroupItemTemplate {
    group: groups::Model,
}

#[derive(Template)]
#[template(path = "group_form.html")]
struct GroupFormTemplate {
    friends: Vec<users::Model>,
}

#[derive(Template)]
#[template(path = "group_detail.html")]
struct GroupDetailTemplate {
    group: groups::Model,
}

#[derive(Deserialize)]
pub struct NewGroupForm {
    name: String,

    #[serde(default)]
    friend_ids: Vec<i32>,
}

pub async fn list_groups(State(state): State<AppState>) -> impl IntoResponse {
    match get_all_groups(&state.db).await {
        Ok(groups) => match (GroupsTemplate { groups }).render() {
            Ok(html) => Html(html).into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Error: {e}")).into_response(),
        },
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Error: {e}")).into_response(),
    }
}

// funkcija, ki vrne obrazec za ustvarjanje nove skupine
pub async fn group_form(State(state): State<AppState>) -> impl IntoResponse {
    match get_all_users(&state.db).await {
        Ok(friends) => {
            let template = GroupFormTemplate { friends };

            match template.render() {
                // predlogo spremeni v HTML String (vrne Result<String, Err>)
                Ok(html) => Html(html).into_response(),
                Err(e) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, format!("Error: {e}")).into_response()
                }
            }
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error while loading friends: {e}"),
        )
            .into_response(),
    }
}

pub async fn add_group(
    State(state): State<AppState>,
    Form(form): Form<NewGroupForm>,
) -> impl IntoResponse {
    match create_group(&state.db, &form.name).await {
        Ok(group) => {
            for friend_id in &form.friend_ids {
                if let Err(e) = add_member_to_group(&state.db, group.id, *friend_id).await {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Error while adding member to group: {e}"),
                    )
                        .into_response();
                }
            }

            match (GroupItemTemplate { group }).render() {
                Ok(html) => Html(html).into_response(),
                Err(e) => {
                    (StatusCode::INTERNAL_SERVER_ERROR, format!("Error: {e}")).into_response()
                }
            }
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error while adding group: {e}"),
        )
            .into_response(),
    }
}

pub async fn group_detail(State(state): State<AppState>, Path(id): Path<i32>) -> impl IntoResponse {
    match find_group_by_id(&state.db, id).await {
        Ok(Some(group)) => match (GroupDetailTemplate { group }).render() {
            Ok(html) => Html(html).into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Error: {e}")).into_response(),
        },
        Ok(None) => (StatusCode::NOT_FOUND, "Group not found").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Error: {e}")).into_response(),
    }
}
