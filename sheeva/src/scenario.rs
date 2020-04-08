use crate::command::Commands;
use crate::parser::TreeParser;
use crate::tree::Tree;
use regex::Regex;

pub struct Scenario {
    name: String,
    tree: Tree,
    commands: Option<Commands>,
}

impl Scenario {
    pub fn new(filename: &str) -> Option<Scenario> {
        let parser = TreeParser::new(filename);
        if let Some(tree) = parser.read() {
            if let Some(node) = tree.dfs().skip(1).next() {
                let re = Regex::new(r"Сценарий (.*)").unwrap();
                if let Some(name) = re.captures(node.name()).unwrap().get(1) {
                    return Some(Scenario {
                        name: String::from(name.as_str()),
                        tree: tree,
                        commands: None,
                    });
                }
            }
        }
        None
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn load_commands(&mut self, cmds: Commands) {
        self.commands = Some(cmds);
    }

    pub async fn execute(&self) {
        for node in self.tree.dfs() {
            for command in node.values().iter() {
                println!("  Executing command {}", command);
                if let Some(executor) = &self.commands {
                    executor.execute(command).await;
                }
            }
        }
    }
}
