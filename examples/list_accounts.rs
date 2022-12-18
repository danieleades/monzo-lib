use clap::Parser;
use monzo::Client;

#[derive(Parser)]
struct Args {
    access_token: String,
}

#[tokio::main]
async fn main() -> monzo::Result<()> {
    let args = Args::parse();
    let client = Client::new(args.access_token);

    for account in client.accounts().await? {
        println!("{account:#?}");
    }

    Ok(())
}
