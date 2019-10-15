use reqwest::Client as HttpClient;


fn endpoint(endpoint: impl AsRef<str> + 'static) -> String {
    format!("https://api.monzo.com/{}", endpoint.as_ref())
}

mod accounts;
mod balance;
use self::accounts::Accounts;
use self::balance::Balance;
use crate::Result;
use serde::de::DeserializeOwned;

#[derive(Default)]
pub struct ClientBuilder {
    access_token: Option<String>,
    refresh_token: Option<String>,
}

impl ClientBuilder {
    pub fn access_token(mut self, access_token: impl Into<String>) -> Self {
        self.access_token = Some(access_token.into());
        self
    }

    pub fn refresh_token(mut self, refresh_token: impl Into<String>) -> Self {
        self.refresh_token = Some(refresh_token.into());
        self
    }

    pub fn build(self) -> Client {
        // for now, assume we have both
        let access_token = self.access_token.unwrap();
        let refresh_token = self.refresh_token;

        let http_client = HttpClient::new();

        Client {
            http_client,
            access_token,
            refresh_token,
        }
    }
}

/// Monzo Client
pub struct Client {
    http_client: HttpClient,
    access_token: String,
    refresh_token: Option<String>,
}

impl Client {
    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }

    async fn get<T: DeserializeOwned>(&self, endpoint: impl AsRef<str>) -> Result<T> {
        Ok(self
            .http_client
            .get(endpoint.as_ref())
            .bearer_auth(&self.access_token)
            .send()
            .await?
            .json()
            .await?)
    }

    pub async fn accounts(&self) -> Result<Accounts> {
        self.get(endpoint("accounts")).await
    }

    pub async fn balance(&self, account_id: impl AsRef<str>) -> Result<Balance> {
        let response = self
            .http_client
            .get(&endpoint("balance"))
            .bearer_auth(&self.access_token)
            .query(&[("account_id", account_id.as_ref())])
            .send()
            .await?;

        Ok(response.json().await?)
    }
}
