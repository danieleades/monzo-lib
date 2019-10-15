[![Build Status](https://travis-ci.org/danieleades/monzo-lib.svg?branch=master)](https://travis-ci.org/danieleades/monzo-lib)

# monzo-lib

This crate is an async Monzo API client in pure rust.

It is intended as the backend of a monzo CLI app that i'll probably
never get to building.

In order to use this client, you will first need to get an access token and/or refresh token for the Monzo API (see [the docs](https://docs.monzo.com/))

### Usage
```rust
use monzo_lib::{Client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::builder()
        .access_token("ACCESS_TOKEN")
        .refresh_token("REFRESH_TOKEN")
        .build();

    let accounts = client.accounts().await?;

    let account_id = &accounts[0].id;

    let balance = client.balance(account_id).await?;

    Ok(())
}
```

License: Apache-2.0
