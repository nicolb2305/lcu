pub mod actions;
pub mod client;
pub mod endpoints;
pub mod types;

#[cfg(test)]
mod tests {
    use crate::{actions::get_online_friends, client::Client};
    use eyre::Result;

    #[test]
    fn print_friends() -> Result<()> {
        let client = Client::new()?;
        let friends = get_online_friends(&client)?;
        dbg!(friends);
        Ok(())
    }
}
