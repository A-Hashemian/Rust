
use substrate_subxt::{balances::{Balances, AccountData}, system::System};
use subxt::{Client, ClientBuilder, PairSigner};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new Substrate client
    let client = ClientBuilder::<PairSigner>::new()
        .set_url("wss://rpc.polkadot.io")
        .build()
        .await?;
    
// Retrieve the balance of an account
    let account_id = System::fetch_account_id(&client, &"Alice".into()).await?;
    let account_data = Balances::fetch_account_data(&client, &account_id).await?;
    
        // Print the account balance
    println!("Account balance: {:?}", account_data.free);

    Ok(())
    
   /*
   This example is used to query the balance of an account on the Polkadot network using the Substrate and Subxt libraries.

A new Substrate client is created with ClientBuilder and configured with the requested network URL. Next, 
System::fetch_account_id retrieves the account ID of a specific user, 
and Balances::fetch_account_data retrieves account information.
   */
