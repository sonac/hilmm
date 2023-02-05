extern crate dotenv;

use rocket::serde::json::serde_json;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YahooResp {
  pub option_chain: Options
}

#[derive(Debug, Deserialize)]
pub struct Options {
	pub result: Vec<StockData>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StockData {
	pub underlying_symbol: String,
	pub quote: Quote
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Quote {
  pub regular_market_price: f64
}

pub struct ReqwestHttpClient {
  client: reqwest::Client,
}

impl Default for ReqwestHttpClient {
  fn default() -> Self {
      let mut headers = reqwest::header::HeaderMap::new();
      headers.insert(reqwest::header::USER_AGENT, reqwest::header::HeaderValue::from_static("Mozilla/5.0...."));
      let client = reqwest::Client::builder().default_headers(headers).build().ok().unwrap();
      ReqwestHttpClient { client: client }
  }
}

impl ReqwestHttpClient {
  pub async fn fetch_price(&self, ticker: String) -> Option<YahooResp> {
    let mut url = "https://query1.finance.yahoo.com/v7/finance/options/".to_owned();
    url.push_str(ticker.as_str());
    let resp = self.client.get(url).send().await;
    match resp {
      Ok(r) => {
        let res = r.text().await;
        match res {
          Ok(r_text) => {
            match serde_json::from_str::<YahooResp>(&r_text) {
              Ok(yr) => {
                Some(yr)
              }
              Err(e) => {
                println!("error occurred {}", e);
                None
              }
            }
          }
          Err(e) => {
            println!("error occurred, {}", e);
            None
          }
        }
      }
      Err(e) => {
        println!("error occured {:?}", e);
        None
      }
    }
  }
}
