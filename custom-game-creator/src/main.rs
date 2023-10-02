use client_api::{actions::create_custom, client::Client};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::new()?;
    Ok(create_custom(&client).await?)
}
