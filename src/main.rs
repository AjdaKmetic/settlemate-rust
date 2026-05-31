/* use sea_orm::{EntityTrait};

use settlemate_rust::{
    // models::expense::Expense,
    // models::group::Group,
    // services::split::Split,
    // models::user::{User},
    database::connect,
    entities::users,
    services::user_service::create_user,
};

#[tokio::main]
async fn main() {

    /*
    let janez = User::new(1, "Janez Novak", "janeznovak@example.com");
    let marija = User::new(2, "Marija Novak", "marijanovak@example.com");

    let mut group = Group::new(1, "Amsterdam");

    group.add_member(janez.id);
    group.add_member(marija.id);

    let expenses = vec![
        Expense::new(
            1,
            "Hotel".into(),
            200.0,
            janez.id,
            Some(group.id),
            Split::Equal(vec![janez.id, marija.id]),
        ),
        Expense::new(
            2,
            "Vecerja".into(),
            100.0,
            marija.id,
            Some(group.id),
            Split::Exact(vec![(janez.id, 70.0), (marija.id, 30.0)]),
        ),
    ];

    let balances =
        settlemate_rust::services::balance::Balance::calculate_balances(&expenses);

    println!("Balances: {:?}", balances);

    let transactions =
        settlemate_rust::services::simplify::simplify_debts(&balances);

    println!("Simplified Transactions: {:?}", transactions);
    */

    let db = connect()
        .await
        .expect("Povezava z bazo ni uspela");

    println!("Povezava z bazo deluje.");

/*    let new_user = users::ActiveModel {
        name: Set("Janez Novak".to_string()),
        email: Set("janez@example.com".to_string()),
        ..Default::default()
    };

    let result = new_user
        .insert(&db)
        .await
        .expect("Dodajanje uporabnika ni uspelo");

    println!("Dodan uporabnik: {:?}", result);
*/

    let result = create_user(
        &db,
        "Janez Novak",
        "janez@example.com",
        "janez123"
    )
    .await
    .expect("Dodajanje uporabnika ni uspelo");

    let all_users = users::Entity::find()
        .all(&db)
        .await
        .expect("Branje uporabnikov ni uspelo");

    println!("Vsi uporabniki v bazi:");
    for user in all_users {
        println!("{} - {} ({})", user.id, user.name, user.email);
    }

} */
<<<<<<< HEAD

/* ko bova imeli podelane handlerje 

use axum::{Router, routing::get};
use settlemate_rust::app::state::{AppState, seed_demo};
use settlemate_rust::database::connect;
// use settlemate_rust::handlers::groups::list_group_members;
use settlemate_rust::handlers::index::index;
// use settlemate_rust::handlers::users::{create_user_handler, list_users};
=======
mod handlers;

use axum::{routing::get, Router};
use handlers::index::index;
use handlers::account::account;
use tower_http::services::ServeDir;
use settlemate_rust::handlers::dashboard::dashboard;
use settlemate_rust::handlers::activity::activity;
>>>>>>> e8bf4b3c25b99be1a85441e06423537f278d5b91

#[tokio::main]
async fn main() {
    let app = Router::new()
    .route("/", get(index))
    .route("/account", get(account))
    .route("/dashboard", get(dashboard))
    .route("/activity", get(activity))
    .nest_service("/static", ServeDir::new("static"));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
*/

fn main() {

}



