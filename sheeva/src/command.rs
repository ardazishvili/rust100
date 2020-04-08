use crate::iterators::Node;
use crate::parser::TreeParser;
use regex::Regex;
use reqwest::Client;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Command {
    name: String,
    command: String,
}

impl Command {
    pub fn new(name: String, command: String) -> Command {
        Command { name, command }
    }

    pub fn query(&self) -> &str {
        &self.command
    }
}

pub struct Commands {
    commands: HashMap<String, Vec<String>>,
}

#[derive(Debug)]
pub enum ExeStatus {
    OK,
    COMMAND_NOT_FOUND,
    HTTP_ERROR,
}

impl Commands {
    pub fn new(filename: &str) -> Commands {
        let mut commands = HashMap::new();

        let parser = TreeParser::new(filename);
        let re = Regex::new(r"Команда (.*)").unwrap();
        if let Some(tree) = parser.read() {
            // skip root or tree to include only commands
            for element in tree.dfs().skip(1) {
                if let Some(name) = re.captures(element.name()).unwrap().get(1) {
                    commands.insert(String::from(name.as_str()), element.values().clone());
                }
            }
        }

        Commands { commands }
    }

    pub async fn execute(&self, command: &str) -> ExeStatus {
        let client = Client::new();
        if let Some(queries) = self.commands.get(command) {
            for query in queries {
                println!("      Executing query {}", query);
                match client.get(query).send().await {
                    Ok(response) => response.status(),
                    Err(_) => return ExeStatus::HTTP_ERROR,
                };
            }
        } else {
            return ExeStatus::COMMAND_NOT_FOUND;
        };

        ExeStatus::OK
    }
}
