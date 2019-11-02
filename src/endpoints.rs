use crate::{Error, Result};

pub mod accounts;
pub mod auth;
pub mod balance;
pub mod feed_items;
pub mod pots;
pub mod transactions;

pub(crate) async fn handle_response<T>(request_builder: reqwest::RequestBuilder) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    let response = request_builder.send().await?;

    dbg!("got this far");
    match response.status() {
        x if x.is_success() => debug_deserialise(response).await,
        x if x.is_client_error() || x.is_server_error() => {
            println!("response.body: {:#?}", response.text().await?);
            Err(Error::from(x))
        }
        _ => unreachable!(),
    }
}

#[cfg(debug_assertions)]
async fn debug_deserialise<T>(response: reqwest::Response) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    println!("response.code: {:#?}", response.status());
    let json: serde_json::Value = response.json().await?;
    let pretty_string = serde_json::to_string_pretty(&json).unwrap();

    println!("\nresponse.body: {}", &pretty_string);

    Ok(serde_json::from_str(&pretty_string).expect("deserialising from string failed"))

    /* Ok(response.json().await?) */
}
