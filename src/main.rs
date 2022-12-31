mod api;
mod models;
mod repository;

#[macro_use] 
extern crate rocket;

use api::user_api::{get_user, update_user, delete_user};
use api::auth_api::{signup, signin, validate};
use repository::mongo_repo::MongoDB;

#[launch]
fn rocket() -> _ {
    println!("Starting server");
    let db = MongoDB::init();
    rocket::build().manage(db).mount("/", routes![signup, signin, validate, get_user, update_user, delete_user])
}
