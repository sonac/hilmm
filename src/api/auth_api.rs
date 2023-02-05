use argon2::{self, Config};
use rocket::{http::{Status, CookieJar, Cookie}, serde::json::Json, State};


use crate::{models::{portfolio_model::Portfolio, user_model::{AuthInfo, User}}, repository::mongo_repo::MongoDB};

fn hash_password(pwd: &String) -> String {
    let salt = b"somesalt";
    let config = Config::default();
    argon2::hash_encoded(pwd.as_bytes(), salt, &config).unwrap()
}

fn check_pwd(pwd: &String, hash: &String) -> bool {
    println!("{}", pwd);
    println!("{}", hash);
    let res = argon2::verify_encoded(hash, pwd.as_bytes()).unwrap();
    println!("{}", res);
    res
}

#[post("/signup", data = "<auth_info>")]
pub fn signup(
    db: &State<MongoDB>,
    auth_info: Json<AuthInfo>,
    cookies: &CookieJar<'_>
) -> Result<Json<String>, Status> {
    let empty_portfolio = Portfolio{
        id: None,
        user_assets: Vec::new(),
        buys: Vec::new()
    };
    let data = User {
        id: None,
        email: auth_info.email.to_owned(),
        pwd: hash_password(&auth_info.pwd.to_owned()),
        portfolio: empty_portfolio,
    };
    let res = db.create_user(data);
    match res {
        Some(user_id) => {
            cookies.add_private(Cookie::new("user_id", user_id));
            Ok(Json(String::from("success")))   
        }
        None => Err(Status::BadRequest),
    }
}

#[post("/signin", data = "<auth_info>")]
pub fn signin(
    db: &State<MongoDB>,
    auth_info: Json<AuthInfo>,
    cookies: &CookieJar<'_>
) -> Result<Json<String>, Status> {
    let maybe_user = db.get_user_by_email(&auth_info.email);
    match maybe_user {
        Some(user) => {
            if check_pwd(&auth_info.pwd, &user.pwd) {
                let user_id = user.id.unwrap().to_hex();
                cookies.add_private(Cookie::new("user_id", user_id));
                Ok(Json(user.email))
            } else {
                println!("wrong password!");
                Err(Status::BadRequest)
            }
        }
        None => {
            println!("user is not found!");
            Err(Status::BadRequest)
        }
    }
}

#[get("/validate")]
pub async fn validate(
    db: &State<MongoDB>,
    cookies: &CookieJar<'_>
) -> Result<Json<User>, Status> {
    let maybe_user_id = cookies.get_private("user_id").map(|crumb| format!("{}", crumb.value()));
    match maybe_user_id {
        Some(user_id) => {
            let maybe_user = db.get_user_by_id(&user_id);
            match maybe_user {
                Some(user) => {
                    let upd_user = user.refresh_assets().await;
                    let res = db.update_user(&upd_user);
                    match res {
                        Ok(_) => {
                            println!("successfully updated user");
                            Ok(Json(upd_user))
                        }
                        Err(err) => {
                            println!("{} err occurred", err);
                            Err(Status::InternalServerError)
                        }
                    }
                }
                None => Err(Status::BadRequest)
            }
        }
        None => Err(Status::Forbidden)
    }
}

