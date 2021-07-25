
# Monzo

[![Latest Docs](https://docs.rs/monzo-lib/badge.svg)](https://docs.rs/monzo-lib/)
![Continuous integration](https://github.com/danieleades/monzo-lib/workflows/Continuous%20integration/badge.svg)

This crate is a Monzo client in pure rust.

It's ergonomic, strongly-typed, and asynchronous.

In order to use this client, you will first need to get an access token and/or refresh token for the Monzo API (see [the monzo API docs](https://docs.monzo.com/))

## Usage
```rust no_run
use monzo::Client;

#[tokio::main]
async fn main() -> monzo::Result<()> {
    // You can create a simple monzo client using only an access token
    let quick_client = Client::new("ACCESS_TOKEN");

    // get a list of accounts
    let accounts = quick_client.accounts().await?;

    // get the id of one of the accounts
    let account_id = &accounts[0].id;

    // get the balance of that account
    let balance = quick_client.balance(account_id).await?;

    // If you have a refresh token and client credentials
    // you can create or upgrade a client which is capable
    // of refreshing its own access token.
    let mut refreshable_client =
        quick_client.with_refresh_tokens("CLIENT_ID", "CLIENT_SECRET", "REFRESH_TOKEN");

    refreshable_client.refresh_auth().await?;

    Ok(())
}
```

## Contributing

see the following issue tags for good starting points for contributions
 - [Good First Issue](https://github.com/danieleades/monzo-lib/labels/good%20first%20issue)
 - [Help Wanted](https://github.com/danieleades/monzo-lib/labels/help%20wanted)

 or checkout the [project board](https://github.com/danieleades/monzo-lib/projects)

There are a couple of small gaps in the API surface, but the majority of the endpoints are already supported. If you need a piece of
functionality that is not yet implemented, please open an issue or even
better, a pull request.

---

License: Apache-2.0
