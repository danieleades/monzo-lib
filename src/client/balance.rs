use super::request::Request;
use serde::Deserialize;

/// The balance of a Monzo Account
#[derive(Deserialize, Debug)]
pub struct Balance {
    /// The account balance, in the minor units of the listed currency. ie for
    /// GBP, the balance is in pence.
    pub balance: i64,

    /// The total account balance. I haven't figured out what the difference is
    /// yet
    pub total_balance: i64,

    /// three-letter currency code for the account
    pub currency: String,

    /// total expenditure so far this calendar day
    pub spend_today: i64,
}

// Since there are no fields to set on this request, we simply forward the
// underlying 'Request'
pub(crate) type BalanceRequest = Request<Balance>;
