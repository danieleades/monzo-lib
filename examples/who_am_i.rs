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

    let response = client.who_am_i().await?;

    println!("{:#?}", &response);

    Ok(())
}
