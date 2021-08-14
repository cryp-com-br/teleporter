use std::convert::Infallible;
use warp::{self};

use super::{establish_connection, find_market_ticker};

pub async fn get_market_ticker(
  exchange: String,
  market_type: String,
  symbol: String
) -> Result<Box<dyn warp::Reply>, Infallible> {
  let conn = establish_connection();
  let market_ticker = find_market_ticker(&conn, exchange, market_type, symbol);

  Ok(Box::new(warp::reply::json(&market_ticker)))
}