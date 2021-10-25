use chrono::{Duration, Utc};
use clap::Parser;
use monzo::Client;

#[derive(Parser)]
struct Args {
    access_token: String,
    #[clap(long, default_value = "2")]
    limit: u16,
    #[clap(long, default_value = "89")]
    days: i64,
}

#[tokio::main]
async fn main() -> monzo::Result<()> {
    let args = Args::parse();
    let client = Client::new(args.access_token);

    let accounts = client.accounts().await?;
    let account_id = &accounts[0].id;

    let transactions = client
        .transactions(account_id)
        .since(Utc::now() - Duration::days(args.days))
        .limit(args.limit)
        .send()
        .await?;

    println!("account: {}", account_id);
    transactions.iter().for_each(|t| println!("\t{}", t.id));

    Ok(())
}
