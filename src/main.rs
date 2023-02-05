mod api;
mod models;
mod repository;
mod http;

#[macro_use] 
extern crate rocket;

use api::user_api::{get_user, update_user, delete_user};
use api::auth_api::{signup, signin, validate};
use api::portfolio_api::add_buy;

use repository::mongo_repo::MongoDB;

#[get("/healthcheck")]
fn healthcheck() -> &'static str {
    "Ok"
}

#[launch]
fn rocket() -> _ {
    println!("Starting server");
    let db = MongoDB::init();
    rocket::build().manage(db).mount("/", routes![healthcheck, signup, signin, validate, get_user, update_user, delete_user, add_buy])
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::http::Status;

    #[test]
    fn test_healthcheck() {
        use rocket::local::blocking::Client;

        let client = Client::tracked(rocket()).unwrap();
        let response = client.get("/healthcheck").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string(), Some("Ok".into()));
    }
}