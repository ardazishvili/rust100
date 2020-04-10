use reqwest::Error;
use sheeva::*;

fn main() -> Result<(), Error> {
    let commands = Expressions::new("assets/funcs.txt");

    Scenario::new("assets/scenarios/complex.txt").map_or(
        println!("Can't get a scenario"),
        |mut scenario| {
            scenario.load_commands(commands);
            println!("Scenario name: {}", scenario.name());
            scenario.execute();
            // scenario.print();
        },
    );

    Ok(())
}
