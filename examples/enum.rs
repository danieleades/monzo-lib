//! For convenience, it's be possible to wrap the multiple client types in a
//! single enum.

use monzo::{
    inner_client::{Quick, Refreshable},
    Account, Client, Result,
};

#[allow(dead_code)]
enum MonzoClient {
    Quick(Client<Quick>),
    Refreshable(Client<Refreshable>),
}

impl MonzoClient {
    async fn accounts(&self) -> Result<Vec<Account>> {
        match self {
            Self::Quick(client) => client.accounts().await,
            Self::Refreshable(client) => client.accounts().await,
        }
    }
}

#[tokio::main]
async fn main() {
    let client = Client::new("access_token");

    let monzo_client = MonzoClient::Quick(client);

    let accounts = monzo_client.accounts().await.unwrap();

    for account in accounts {
        println!("{account:#?}");
    }
}
