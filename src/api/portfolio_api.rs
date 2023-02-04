use chrono;

use rocket::{
    http::{CookieJar, Status},
    serde::json::Json,
    State,
};

use crate::{
    models::{
        portfolio_model::{Asset, Buy, BuyInfo},
        user_model::User,
    },
    repository::mongo_repo::MongoDB,
};

#[post("/asset", data = "<buy>")]
pub fn add_buy(
    db: &State<MongoDB>,
    buy: Json<BuyInfo>,
    cookies: &CookieJar<'_>,
) -> Result<Json<User>, Status> {
    let maybe_user_id = cookies
        .get_private("user_id")
        .map(|crumb| format!("{}", crumb.value()));
    match maybe_user_id {
        Some(user_id) => {
            let maybe_user: Option<User> = db.get_user_by_id(&user_id);
            match maybe_user {
                Some(mut user) => {
                    let user_buy = to_user_buy(buy.0);
                    let user_asset = user_buy.to_user_asset();
                    user.portfolio.buys.push(user_buy);
                    user.portfolio = user.portfolio.add_asset(user_asset);
                    println!("{:?}", user);
                    let res = db.update_user(&user);
                    match res {
                        Ok(_) => {
                            println!("successfully updated user");
                            Ok(Json(user))
                        }
                        Err(err) => {
                            println!("{} err occurred", err);
                            Err(Status::InternalServerError)
                        }
                    }
                }
                None => Err(Status::InternalServerError),
            }
        }
        None => Err(Status::Forbidden),
    }
}

fn fetch_price(_ticker: &String) -> f32 {
    0.0
}

fn to_user_buy(buy_info: BuyInfo) -> Buy {
    let asset_price = fetch_price(&buy_info.ticker);
    let asset = Asset {
        name: buy_info.name,
        ticker: buy_info.ticker,
        price: asset_price,
        id: None,
    };
    Buy {
        id: None,
        asset: asset,
        amount: buy_info.amount,
        paid: buy_info.paid,
        date: chrono::Utc::now(),
    }
}
