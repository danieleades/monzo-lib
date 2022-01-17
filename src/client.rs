//! Monzo API clients

use async_trait::async_trait;
use serde::{de::DeserializeOwned, Deserialize};
use tracing::instrument;

use crate::{
    endpoints::{accounts, balance, feed_items, pots, transactions, who_am_i, Endpoint},
    Result,
};

pub mod inner;

/// A generic trait of any HTTP client which also stores and manages an access
/// token.
#[async_trait]
pub trait Inner: Send + Sync + std::fmt::Debug {
    /// Construct end send an HTTP request using the provided Endpoint with
    /// bearer token authentication.
    async fn execute(&self, endpoint: &dyn Endpoint) -> reqwest::Result<reqwest::Response>;

    /// Return a reference to the current access token
    fn access_token(&self) -> &String;

    /// Manually set the access token
    fn set_access_token(&mut self, access_token: String);

    /// The base URL of the API
    fn url(&self) -> &str;
}

/// A Monzo API client
#[derive(Debug)]
pub struct Client<M>
where
    M: Inner,
{
    inner_client: M,
}

impl<M> Client<M>
where
    M: Inner,
{
    /// Return a reference to the current access token
    #[must_use]
    pub fn access_token(&self) -> &String {
        self.inner_client.access_token()
    }

    /// Manually update the access token
    pub fn set_access_token(&mut self, access_token: impl Into<String>) {
        self.inner_client.set_access_token(access_token.into());
    }

    /// Return a list of accounts
    ///
    /// # Example
    /// ```no_run
    /// # use monzo::{Client};
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// #
    /// # let ACCESS_TOKEN = "ACCESS TOKEN";
    /// # let client = Client::new(ACCESS_TOKEN);
    /// #
    /// let accounts = client.accounts().await?;
    /// #
    /// # Ok(())
    /// # }
    pub async fn accounts(&self) -> Result<Vec<accounts::Account>> {
        #[derive(Deserialize)]
        pub(crate) struct Response {
            accounts: Vec<accounts::Account>,
        }
        let response: Response = handle_request(&self.inner_client, &accounts::List).await?;

        Ok(response.accounts)
    }

    /// Return the balance of a given account
    ///
    /// # Example
    /// ```no_run
    /// # use monzo::{Client};
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// #
    /// # let ACCESS_TOKEN = "ACCESS TOKEN";
    /// # let ACCOUNT_ID = "ACCOUNT_ID";
    /// # let client = Client::new(ACCESS_TOKEN);
    /// #
    /// let balance = client.balance(ACCOUNT_ID).await?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    pub async fn balance(&self, account_id: &str) -> Result<balance::Balance> {
        handle_request(&self.inner_client, &balance::Get::new(account_id)).await
    }

    /// Return a list of Pots
    ///
    /// # Example
    /// ```no_run
    /// # use monzo::{Client};
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// #
    /// # let ACCESS_TOKEN = "ACCESS TOKEN";
    /// # let ACCOUNT_ID = "ACCOUNT_ID";
    /// #
    /// # let client = Client::new(ACCESS_TOKEN);
    /// #
    /// let pots = client.pots(ACCOUNT_ID).await?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    pub async fn pots(&self, account_id: &str) -> Result<Vec<pots::Pot>> {
        #[derive(Deserialize)]
        struct Response {
            pots: Vec<pots::Pot>,
        }

        let response: Response =
            handle_request(&self.inner_client, &pots::List::new(account_id)).await?;

        Ok(response.pots)
    }

    /// Post a basic item on the account feed.
    ///
    /// # Example
    /// ```no_run
    /// # use monzo::Client;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let access_token = "ACCESS_TOKEN";
    /// # let client = Client::new(access_token);
    /// #
    /// let account_id = "ACCOUNT_ID";
    /// let title = "Feed Item Title";
    /// let image_url = "http://www.nyan.cat/cats/original.gif";
    ///
    /// client
    ///     .basic_feed_item(account_id, title, image_url)
    ///     .body("i figured out how to send messages to monzo from my computer...")
    ///     .send()
    ///     .await?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    pub fn basic_feed_item<'a>(
        &'a self,
        account_id: &'a str,
        title: &'a str,
        image_url: &'a str,
    ) -> feed_items::basic::Request<'a> {
        feed_items::basic::Request::new(&self.inner_client, account_id, title, image_url)
    }

    /// Deposit money into a pot
    pub async fn deposit_into_pot(
        &self,
        pot_id: &str,
        source_account_id: &str,
        amount: u32,
    ) -> Result<pots::Pot> {
        handle_request(
            &self.inner_client,
            &pots::Deposit::new(pot_id, source_account_id, amount),
        )
        .await
    }

    /// Withdraw money from a pot
    pub async fn withdraw_from_pot(
        &self,
        pot_id: &str,
        destination_account_id: &str,
        amount: u32,
    ) -> Result<pots::Pot> {
        handle_request(
            &self.inner_client,
            &pots::Withdraw::new(pot_id, destination_account_id, amount),
        )
        .await
    }

    /// Get a list of transactions
    ///
    /// The only required field is the account id, however optional pagination
    /// parameters can be supplied.
    ///
    /// # Example
    /// ```no_run
    /// use chrono::{Duration, Utc};
    /// use monzo::Client;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let access_token = "ACCESS_TOKEN";
    /// # let client = Client::new(access_token);
    /// #
    /// let account_id = "ACCOUNT_ID";
    ///
    /// let transactions = client
    ///     .transactions(account_id)
    ///     .since(Utc::now() - Duration::days(10))
    ///     .limit(10)
    ///     .send()
    ///     .await?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Note
    /// *The Monzo API will only return transactions from more than 90 days ago
    /// in the first 5 minutes after authorising the Client. You can avoid this
    /// by using the 'since' method.*
    pub fn transactions<'a>(&'a self, account_id: &'a str) -> transactions::List<'a> {
        transactions::List::new(&self.inner_client, account_id)
    }

    /// Retrieve a transaction by transaction id
    ///
    /// # Example
    /// ```no_run
    /// use monzo::Client;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let access_token = "ACCESS_TOKEN";
    /// # let client = Client::new(access_token);
    /// #
    /// let transaction_id = "TRANSACTION_ID";
    ///
    /// let transactions = client.transaction(transaction_id).send().await?;
    /// #
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Note
    /// *The Monzo API will only return transactions from more than 90 days ago
    /// in the first 5 minutes after authorising the Client.*
    pub fn transaction<'a>(&'a self, transaction_id: &'a str) -> transactions::Get<'a> {
        transactions::Get::new(&self.inner_client, transaction_id)
    }

    /// Return information about the current session
    pub async fn who_am_i(&self) -> Result<who_am_i::Response> {
        handle_request(&self.inner_client, &who_am_i::Request).await
    }
}

#[instrument(skip(client, endpoint), fields(url = client.url(), endpoint = endpoint.endpoint()))]
pub async fn handle_request<R>(client: &dyn Inner, endpoint: &dyn Endpoint) -> Result<R>
where
    R: DeserializeOwned,
{
    tracing::info!("sending request");
    let response = client.execute(endpoint).await?;
    tracing::info!("response received");

    let result = handle_response(response).await;

    match &result {
        Ok(_) => {
            tracing::info!("request successful");
        }
        Err(e) => {
            tracing::info!("request failed: {}", e);
        }
    };
    result
}

async fn handle_response<R>(response: reqwest::Response) -> Result<R>
where
    R: DeserializeOwned,
{
    let status = response.status();

    if status.is_success() {
        Ok(serde_json::from_slice(&response.bytes().await?)?)
    } else {
        Err(status.into())
    }
}
