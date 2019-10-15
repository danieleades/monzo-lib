use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Accounts {
    accounts: Vec<Account>,
}

impl std::ops::Deref for Accounts {
    type Target = Vec<Account>;
    fn deref(&self) -> &Self::Target {
        &self.accounts
    }
}

#[derive(Deserialize, Debug)]
pub struct Account {
    pub id: String,
    pub description: String,
    pub created: DateTime<Utc>,
}
