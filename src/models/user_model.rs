use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

use crate::http::client;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub email: String,
    pub pwd: String, 
    pub portfolio: super::portfolio_model::Portfolio
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthInfo {
    pub email: String,
    pub pwd: String
}

impl User {
    pub async fn refresh_assets(mut self) -> User {
        let client = client::ReqwestHttpClient::default();
        for idx in 0..self.portfolio.user_assets.len() {
            let price = client.fetch_price(self.portfolio.user_assets[idx].asset.ticker.clone());
            match price.await {
                Some(p) => {
                    if p.option_chain.result.len() == 0 {
                        println!("no price in the result");
                        continue
                    }
                    let price = p.option_chain.result[0].quote.regular_market_price;
                    self.portfolio.user_assets[idx].asset.price = price;
                    self.portfolio.user_assets[idx].current_value = price * self.portfolio.user_assets[idx].amount;
                }
                None => {
                    println!("no price was fetched")
                }
            }
        }
        self
    }
}