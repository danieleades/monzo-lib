use clap::Clap;
use monzo::Client;

#[derive(Clap)]
struct Args {
    access_token: String,
}

#[tokio::main]
async fn main() -> monzo::Result<()> {
    let args = Args::parse();
    let client = Client::new(args.access_token);

    for account in client.accounts().await? {
        println!("account: {}", &account.id);
        for pot in client.pots(&account.id).await? {
            println!("    {}", pot.name);
        }
    }

    Ok(())
}
