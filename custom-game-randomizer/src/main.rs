use client_api::{actions::randomize_teams, client::Client};
use eyre::Result;

fn main() -> Result<()> {
    let client = Client::new()?;
    randomize_teams(&client)
}
