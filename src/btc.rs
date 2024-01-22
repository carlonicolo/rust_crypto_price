use std::env;
use serde::Deserialize;
use reqwest::*;

/// To deserialize the JSON response
#[derive(Deserialize)]
struct BTCPrice {
    // As the JSON data type has "EUR" as the field name.
    // So, the struct `BTCPrice` must have the fied with the same name.
    // In production code scenario, rust recommends snake_case, hence
    // we would five a name in lowercase which can be treated as uppercase or
    // any custom name as provided in `rename` field
    #[serde(rename="EUR")]
    eur: f32,
}

#[tokio::main]
// Get BTC price in EUR
pub(crate) async fn get_btc_price() -> Result<f32> {
    // path has to be w.r.t `Cargo.toml` file
    dotenv::from_path("./.env").expect("Failed to load .env file");
    let url = env::var("URL").expect("URL var not found");

    let body= reqwest::get(url).await?.json::<BTCPrice>().await?;

    let price_eur = body.eur;

    Ok(price_eur)
}

// main function
pub fn main() {
    // get the price in EUR, or else throws error.
    let price_eur = get_btc_price().expect("Failure in getting BTC price via API request");
    println!("BTC price in EUR: {:.2}", price_eur);
}