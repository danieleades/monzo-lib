use clap::Parser;
use monzo::Client;

#[derive(Parser)]
struct Args {
    #[clap[long, env]]
    client_id: String,

    #[clap[long, env]]
    client_secret: String,

    #[clap[long, env]]
    refresh_token: String,
}

#[tokio::main]
async fn main() -> monzo::Result<()> {
    let args = Args::parse();
    let mut client = Client::new("DUMMY_ACCESS_TOKEN").with_refresh_tokens(
        args.client_id,
        args.client_secret,
        args.refresh_token,
    );

    client.refresh_auth().await?;

    println!("new access token: {}", client.access_token());
    println!("new fresh token: {}", client.refresh_token());

    Ok(())
}
