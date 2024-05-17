use client_api::{
    actions::{create_custom, DraftType, Map},
    client::Client,
};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::new()?;
    Ok(create_custom(&client, DraftType::TorunamentDraft, Map::SummonersRift).await?)
}
