use crate::{models::{portfolio_model::Portfolio, user_model::User}, repository::mongo_repo::MongoDB};
use mongodb::bson::oid::ObjectId;
use rocket::{http::Status, serde::json::Json, State};

#[get("/user/<id>")]
pub fn get_user(db: &State<MongoDB>, id: String) -> Result<Json<User>, Status> {
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let user_detail = db.get_user_by_id(&id);
    match user_detail {
        Some(user) => Ok(Json(user)),
        None => Err(Status::InternalServerError),
    }
}

#[put("/user/<id>", data = "<user>")]
pub fn update_user(db: &State<MongoDB>, id: String, user: Json<User>) -> Result<Json<User>, Status> {
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let empty_portfolio = Portfolio{
        id: None,
        user_assets: Vec::new(),
        buys: Vec::new()
    };
    let data = User {
        id: Some(ObjectId::parse_str(&id).unwrap()),
        email: user.email.to_owned(),
        pwd: String::new(),
        portfolio: empty_portfolio
    };
    let upd_res = db.update_user(&id, data);
    match upd_res {
        Ok(upd) => {
            if upd.matched_count == 1 {
                let upd_usr_info = db.get_user_by_id(&id);
                return match upd_usr_info {
                    Some(user) => Ok(Json(user)),
                    None => Err(Status::InternalServerError),
                };
            } else {
                return Err(Status::NotFound);
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

#[delete("/user/<id>")]
pub fn delete_user(db: &State<MongoDB>, id: String) -> Result<Json<&str>, Status> {
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let res = db.delete_user(&id);
    match res {
        Ok(r) => {
            if r.deleted_count == 1 {
                return Ok(Json("User successfully deleted"));
            } else {
                return Err(Status::NotFound);
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}
