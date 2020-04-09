use crate::parser::{ParseType, TreeParser};
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

pub struct Expressions {
    commands: HashMap<String, Vec<String>>,
    predicates: HashMap<String, String>,
}

#[derive(Debug)]
pub enum ExeStatus {
    OK,
    CommandNotFound,
    HttpError,
}

impl Expressions {
    pub fn new(filename: &str) -> Expressions {
        let mut commands = HashMap::new();
        let mut predicates = HashMap::new();

        let parser = TreeParser::new(filename, ParseType::Expressions);
        let re_cmd = Regex::new(r"Команда (.*)").unwrap();
        let re_pred = Regex::new(r"Условие (.*)").unwrap();
        if let Some(tree) = parser.read() {
            // skip root or tree to include only commands
            for element in tree.dfs().skip(1) {
                if let Some(capture) = re_cmd.captures(element.name()) {
                    if let Some(command) = capture.get(1) {
                        commands.insert(String::from(command.as_str()), element.values().clone());
                    }
                } else if let Some(capture) = re_pred.captures(element.name()) {
                    if let Some(predicate) = capture.get(1) {
                        predicates.insert(
                            String::from(predicate.as_str()),
                            element.values().get(0).unwrap().clone(),
                        );
                    }
                }
            }
        }
        // println!("Commands len is {}", commands.len());
        // println!("Predicates len is {}", predicates.len());

        Expressions {
            commands,
            predicates,
        }
    }

    pub async fn execute(&self, command: &str) -> ExeStatus {
        let client = Client::new();
        if let Some(queries) = self.commands.get(command) {
            for query in queries {
                println!("      Executing query {}", query);
                match client.get(query).send().await {
                    Ok(response) => response.status(),
                    Err(_) => return ExeStatus::HttpError,
                };
            }
        } else {
            return ExeStatus::CommandNotFound;
        };

        ExeStatus::OK
    }

    pub fn eval_predicate(&self, predicate: &str) -> bool {
        let mapping: HashMap<String, bool> = [
            (String::from("Ложь"), false),
            (String::from("Правда"), true),
        ]
        .iter()
        .cloned()
        .collect();
        let predicate = self.predicates.get(predicate).unwrap();
        mapping.get(predicate).unwrap().clone()
    }
}
