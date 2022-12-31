use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Portfolio {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub user_assets: Vec<UserAsset>,
    pub buys: Vec<Buy>
}

#[derive(Debug, Serialize, Deserialize)]
struct Asset {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    name: String,
    price: f32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserAsset {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    asset: Asset,
    volume: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Buy {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    asset: Asset,
    volume: f32,
    date: DateTime
}
