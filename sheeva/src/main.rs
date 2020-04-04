use reqwest::{Client, Error, StatusCode};
use std::fs;

#[derive(Debug)]
struct Command {
    name: String,
    command: String,
}

fn read_commands(filename: &str) -> Vec<Command> {
    let text = match fs::read_to_string(filename) {
        Ok(r) => r,
        Err(_) => panic!("Can't read specified file"),
    };

    let (mut start, mut name) = (false, String::from(""));
    let mut result: Vec<Command> = vec![];
    let lines = text.lines();
    for line in lines {
        match line {
            "{" => start = true,
            "}" => start = false,

            line => match start {
                true => result.push(Command {
                    name: name.clone(),
                    command: String::from(line),
                }),
                false => name = String::from(line),
            },
        }
    }

    result
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::new();
    let commands = read_commands("assets/text.txt");
    for command in commands {
        let r = match client.get("http://localhost:8000").send().await {
            Ok(response) => response,
            Err(_) => panic!("Error at non-blocking call"),
        };
    }

    Ok(())
}
