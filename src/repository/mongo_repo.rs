extern crate dotenv;

use dotenv::dotenv;
use std::env;

use mongodb::{
    bson::{extjson::de::Error, oid::ObjectId, doc, Document},
    results::{InsertOneResult, UpdateResult, DeleteResult},
    sync::{Client, Collection},
};
use crate::models::{portfolio_model::Portfolio, user_model::User};

pub struct MongoDB {
    users: Collection<User>,
}

impl MongoDB {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("error loading env var"),
        };
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("hilmm");
        let users: Collection<User> = db.collection("users");
        MongoDB { users }
    }

    pub fn create_user(&self, new_user: User) -> Option<String> {
        let empty_portfolio = Portfolio{
            id: None,
            user_assets: Vec::new(),
            buys: Vec::new()
        };
        let new_doc = User{
            id: None,
            pwd: new_user.pwd,
            email: new_user.email,
            portfolio: empty_portfolio,
        };
        let res = self.users.insert_one(new_doc, None).ok();
        match res {
            Some(res) => {
                let id = res.inserted_id.as_object_id().unwrap().to_hex();
                Some(id)
            }
            None => None
        }
    }

    pub fn get_user_by_id(&self, id: &String) -> Option<User> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        self.users.find_one(filter, None).ok().expect("Error fetching user by id from db")
    }

    pub fn get_user_by_email(&self, email: &String) -> Option<User> {
        let filter: Document = doc! {"email": email};
        self.users.find_one(filter, None).ok().expect("Error fetching user by email from db")
    }

    pub fn update_user(&self, id: &String, new_user: User) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set":
                {
                   "id": new_user.id,
                   "pwd": new_user.pwd,
                   "email": new_user.email
                },
        };
        let updated_doc = self.users.update_one(filter, new_doc, None).ok().expect("Error updating user");
        Ok(updated_doc)
    }

    pub fn delete_user(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self.users.delete_one(filter, None).ok().expect("Error deleting user");
        Ok(user_detail)
    }
}

