use client_api::{actions::randomize_teams, client::Client};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::new()?;
    Ok(randomize_teams(&client).await?)
}
