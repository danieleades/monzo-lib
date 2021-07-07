use clap::Clap;
use monzo::Client;
use chrono::{Duration, Utc};

#[derive(Clap)]
struct Args {
    access_token: String,
}

#[tokio::main]
async fn main() -> monzo::Result<()> {
    let args = Args::parse();
    let client = Client::new(args.access_token);

    let accounts = client.accounts().await?;
    let account_id = &accounts[0].id;

    let transactions = client
        .transactions(account_id)
        .since(Utc::now() - Duration::days(89))
        .limit(2)
        .send()
        .await?;

    println!("account: {}", account_id);
    transactions.iter()
        .for_each(|t| println!("\t{}", t.id));

    Ok(())
}
