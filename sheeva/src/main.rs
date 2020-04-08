use reqwest::{Client, Error};
use sheeva::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let commands = Commands::new("assets/funcs.txt");

    if let Some(mut scenario) = Scenario::new("assets/scenarios/simple.txt") {
        scenario.load_commands(commands);
        println!("Scenario name: {}", scenario.name());
        scenario.execute().await;
    }

    Ok(())
}
