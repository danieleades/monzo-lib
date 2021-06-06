use crate::{Error, Result};

pub mod accounts;
pub mod auth;
pub mod balance;
pub(crate) mod feed_items;
pub mod pots;
pub mod transactions;
mod utils;

pub(crate) async fn handle_response<T>(request_builder: reqwest::RequestBuilder) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    let response = request_builder.send().await?;

    match response.status() {
        x if x.is_success() => Ok(response.json().await?),
        x if x.is_client_error() || x.is_server_error() => {
            println!("response.body: {:#?}", response.text().await?);
            Err(Error::from(x))
        }
        _ => unreachable!(),
    }
}
