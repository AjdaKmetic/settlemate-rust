use axum::{Router, routing::get, routing::post};
use tower_http::services::ServeDir;

use migration::{Migrator, MigratorTrait};
use settlemate_rust::app::state::AppState;
use settlemate_rust::database::connect;
use settlemate_rust::handlers::{
    account::account,
    activity::activity,
    auth::{login_form, register_form, register_user},
    expenses::{add_expense, new_expense},
    friends::{add_friend, friend_form, list_friends},
    groups::{add_group, group_detail, group_form, list_groups},
    index::{index, tabs_activity, tabs_friends, tabs_groups},
};

#[tokio::main]
async fn main() {
    let db = connect().await.expect("Povezava z bazo ni uspela.");
    Migrator::up(&db, None)
        .await
        .expect("Migracije niso uspele.");

    let state = AppState::new(db);

    let app = Router::new()
        .route("/", get(index))
        .route("/groups", get(list_groups).post(add_group))
        .route("/groups/new", get(group_form))
        .route("/groups/{id}", get(group_detail))
        .route("/friends/form", get(friend_form))
        .route("/friends", get(list_friends).post(add_friend))
        .route("/tabs/friends", get(tabs_friends))
        .route("/tabs/groups", get(tabs_groups))
        .route("/tabs/activity", get(tabs_activity))
        .route("/account", get(account))
        .route("/activity", get(activity))
        .route("/expenses/new", get(new_expense))
        .route("/expenses", post(add_expense))
        .route("/register", get(register_form).post(register_user))
        .route("/login", get(login_form))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Listening on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
