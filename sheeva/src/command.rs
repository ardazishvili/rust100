use crate::iterators::Node;
use crate::parser::TreeParser;
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
        if let Some(tree) = parser.read() {
            // skip root or tree to include only commands
            for element in tree.dfs().skip(1) {
                commands.insert(String::from(element.name()), element.values().clone());
            }
        }

        Commands { commands }
    }

    pub async fn execute(&self, command: String) -> ExeStatus {
        let client = Client::new();
        for (key, value) in &self.commands {
            println!("{:?}", key);
        }
        if let Some(queries) = self.commands.get(&command) {
            for query in queries {
                let status = match client.get(query).send().await {
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
