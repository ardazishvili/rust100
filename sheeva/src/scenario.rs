use crate::command::{ExeStatus, Expressions};
use crate::iterators::NodeType;
use crate::parser::{ParseType, TreeParser};
use crate::tree::Tree;
use regex::Regex;

pub struct Scenario {
    name: String,
    tree: Tree,
    commands: Option<Expressions>,
}

impl Scenario {
    pub fn new(filename: &str) -> Option<Scenario> {
        let parser = TreeParser::new(filename, ParseType::Scenario);
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

    pub fn load_commands(&mut self, cmds: Expressions) {
        println!("loading commands");
        self.commands = Some(cmds);
    }

    pub fn execute(&self) {
        self.commands.as_ref().map(|evaluator| {
            for node in self.tree.cond_iter(evaluator).skip(1) {
                if node.t == NodeType::Exe {
                    let res = evaluator.execute(node.name());
                    match res {
                        ExeStatus::CommandNotFound => println!("      No query {}", node.name()),
                        ExeStatus::HttpError => println!("      Can't connect to the server"),
                        ExeStatus::OK => println!("         Command {} executed", node.name()),
                    }
                }
            }
        });
    }

    pub fn print(&self) {
        println!("Printing the scenario");
        for node in self.tree.dfs() {
            match &node.t {
                NodeType::None => println!("Type of node {} is None", node.name()),
                NodeType::Exe => println!("Type of node {} is Exe", node.name()),
                NodeType::Condition(opt) => opt.as_ref().map_or_else(
                    || {
                        println!( "Type of node {} is ELSE condition", node.name());
                    },

                    |predicate| {
                        self.commands.as_ref().map(|evaluator| {
                            println!(
                                "Type of node {} is IF Condition with predicate {}, which evaluates to {}",
                                node.name(),
                                predicate,
                                evaluator.eval_predicate(&predicate)
                                );
                        });
                    },
                    ),
            }
            for (index, value) in node.values().iter().enumerate() {
                println!("value # {} is {}", index, value);
            }
        }
    }
}
