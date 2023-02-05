use chrono::{DateTime, Utc};

use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Currency {
    EUR,
    USD,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Portfolio {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub user_assets: Vec<UserAsset>,
    pub buys: Vec<Buy>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Asset {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub ticker: String,
    pub price: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserAsset {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    pub asset: Asset,
    pub amount: f64,
    pub current_value: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Buy {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub asset: Asset,
    pub amount: f64,
    pub paid: f64,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub date: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BuyInfo {
    pub ticker: String,
    pub name: String,
    pub paid: f64,
    pub amount: f64,
    pub currency: Currency,
}

impl UserAsset {
    fn merge(mut self, other: UserAsset) -> UserAsset {
        self.amount += other.amount;
        self.current_value += other.current_value;
        self
    }
}

impl Portfolio {
    pub fn add_asset(mut self, asset: UserAsset) -> Portfolio {
        for idx in 0..self.user_assets.len() {
            if asset.asset.name == self.user_assets[idx].asset.name {
                let updated_asset = self.user_assets[idx].clone().merge(asset);
                self.user_assets[idx] = updated_asset;
                return self;
            }
        }
        self.user_assets.push(asset);
        self
    }
}

impl Buy {
    pub fn to_user_asset(&self) -> UserAsset {
        UserAsset {
            id: None,
            asset: self.asset.clone(),
            amount: self.amount,
            current_value: self.amount * self.asset.price,
        }
    }
}
