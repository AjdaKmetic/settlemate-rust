// Primer: branje iz pomnilniškega stanja (`SharedAppData`).
//
// Ta modul prikazuje, kako handler:
//   - sprejme parameter iz poti (Path<GroupId>),
//   - zaklene Mutex okrog `AppData`,
//   - poišče skupino in razreši ID-je članov v imena.
//
// TODO: ko bodo skupine in člani v bazi (entities + migration), bomo to
// celotno funkcijo zamenjali s SeaORM poizvedbo nad `state.db` — Mutex tukaj
// ne bo več potreben.

use askama::Template;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse},
};

use crate::app::state::AppState;
use crate::models::group::GroupId;

#[derive(Template)]
#[template(path = "group_members.html")]
struct GroupMembersTemplate {
    group_name: String,
    members: Vec<String>,
}

// GET /groups/:id/members
pub async fn list_group_members(
    State(state): State<AppState>,
    Path(group_id): Path<GroupId>,
) -> impl IntoResponse {
    let data = state.data.lock().unwrap();

    let Some(group) = data.groups.iter().find(|g| g.id == group_id) else {
        return (StatusCode::NOT_FOUND, "Skupina ne obstaja").into_response();
    };

    let members: Vec<String> = group
        .members()
        .iter()
        .map(|uid| {
            data.users
                .iter()
                .find(|u| u.id == *uid)
                .map(|u| u.name.clone())
                .unwrap_or_else(|| format!("?({uid})"))
        })
        .collect();

    Html(
        GroupMembersTemplate {
            group_name: group.name().to_string(),
            members,
        }
        .render()
        .unwrap(),
    )
    .into_response()
}
