
use substrate_subxt::{balances::{Balances, AccountData}, system::System};
use subxt::{Client, ClientBuilder, PairSigner};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new Substrate client
    let client = ClientBuilder::<PairSigner>::new()
        .set_url("wss://rpc.polkadot.io")
        .build()
        .await?;
