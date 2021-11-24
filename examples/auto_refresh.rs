use std::future::Future;

use monzo::{inner_client::Refreshable, Balance, Pot};
use serde::{Deserialize, Serialize};
use tokio::sync::{Mutex, RwLock};

fn main() {
    // no op
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Auth {
    access_token: String,
    client_id: String,
    client_secret: String,
    refresh_token: String,
}

#[derive(Debug)]
pub struct Client {
    client: RwLock<monzo::Client<Refreshable>>,
    refresh_lock: Mutex<()>,
}

impl Client {
    pub async fn auth(&self) -> Auth {
        let client = self.client.read().await;

        Auth {
            access_token: client.access_token().to_string(),
            client_id: client.client_id().to_string(),
            client_secret: client.client_secret().to_string(),
            refresh_token: client.refresh_token().to_string(),
        }
    }

    pub async fn balance(&self, account_id: &str) -> monzo::Result<Balance> {
        self.with_retry(|| async { self.client.read().await.balance(account_id).await })
            .await
    }

    pub async fn pots(&self, account_id: &str) -> monzo::Result<Vec<Pot>> {
        self.with_retry(|| async { self.client.read().await.pots(account_id).await })
            .await
    }

    pub async fn withdraw_from_pot(
        &self,
        pot_id: &str,
        destination_account_id: &str,
        amount: u32,
    ) -> monzo::Result<Pot> {
        self.with_retry(|| async {
            self.client
                .read()
                .await
                .withdraw_from_pot(pot_id, destination_account_id, amount)
                .await
        })
        .await
    }

    pub async fn deposit_into_pot(
        &self,
        pot_id: &str,
        source_account_id: &str,
        amount: u32,
    ) -> monzo::Result<Pot> {
        self.with_retry(|| async {
            self.client
                .read()
                .await
                .deposit_into_pot(pot_id, source_account_id, amount)
                .await
        })
        .await
    }

    async fn with_retry<F, Fut, R>(&self, f: F) -> monzo::Result<R>
    where
        F: Fn() -> Fut,
        Fut: Future<Output = monzo::Result<R>>,
    {
        let response = f().await;

        if response.is_err() {
            tracing::warn!("authentication failed, access token may have expired");
            self.refresh_auth().await?;
            return f().await;
        }

        response
    }

    async fn refresh_auth(&self) -> monzo::Result<()> {
        tracing::info!("attempting access token refresh");

        let _refresh_lock = if let Ok(lock) = self.refresh_lock.try_lock() {
            lock
        } else {
            tracing::debug!("another thread is already refreshing auth");
            return Ok(());
        };

        self.client.write().await.refresh_auth().await?;
        tracing::info!("access token refreshed");

        Ok(())
    }
}

impl From<Auth> for Client {
    fn from(auth: Auth) -> Self {
        let client = monzo::Client::new(auth.access_token).with_refresh_tokens(
            auth.client_id,
            auth.client_secret,
            auth.refresh_token,
        );

        Self {
            client: RwLock::new(client),
            refresh_lock: Mutex::new(()),
        }
    }
}
