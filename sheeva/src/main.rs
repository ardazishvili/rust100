use reqwest::{Client, Error};
use sheeva::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::new();
    let mut parser = Parser::new();
    parser.read("assets/text.txt");
    for command in parser.commands() {
        match client.get(command.query()).send().await {
            Ok(response) => response,
            Err(_) => panic!("Error at non-blocking call"),
        };
    }

    Ok(())
}
