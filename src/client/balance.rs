use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Balance {
    balance: i64,
    total_balance: i64,
    currency: String,
    spend_today: i64,
}
