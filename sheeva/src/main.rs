use reqwest::Error;
use sheeva::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let commands = Commands::new("assets/funcs.txt");

    if let Some(mut scenario) = Scenario::new("assets/scenarios/complex.txt") {
        scenario.load_commands(commands);
        println!("Scenario name: {}", scenario.name());
        // scenario.execute().await;
        scenario.print();
    } else {
        println!("Can't get a scenario");
    }

    Ok(())
}
