use reqwest::{Client, Error};
use sheeva::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::new();
    // let mut parser = TreeParser::new("assets/funcs.txt");
    let mut parser = TreeParser::new("assets/scenarios.txt");
    if let Some(tree) = parser.read() {
        for command in tree.bfs() {
            // match client.get(command.query()).send().await {
            //     Ok(response) => response,
            //     Err(_) => panic!("Error at non-blocking call"),
            // };
            println!("name is {}", command.name());
            println!("# of children: {}", command.children().len());
            println!("values are:");
            for value in command.values() {
                println!("      {}", value);
            }
            println!("\n");
        }
    }

    let commands = Commands::new("assets/funcs.txt");
    let res = commands.execute(String::from("Команда Тестовая 1"));
    println!("res is {:?}", res.await);

    Ok(())
}
