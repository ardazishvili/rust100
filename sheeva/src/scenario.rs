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

    pub async fn execute(&self) {
        println!("Executing the scenario");
        for node in self.tree.dfs() {
            println!("  Executing command {}", node.name());
            if let Some(executor) = &self.commands {
                executor.execute(node.name()).await;
            }
        }
    }

    pub async fn print_cond(&self) {
        println!("Printing the scenario");
        if let Some(evaluator) = &self.commands {
            for node in self.tree.cond_iter(evaluator).skip(1) {
                if node.t == NodeType::Exe {
                    println!("  Executing command {}", node.name());
                    let res = evaluator.execute(node.name()).await;
                    if res != ExeStatus::OK {
                        println!("      No query for command {}", node.name());
                    }
                }
            }
        }
    }

    pub fn print(&self) {
        for node in self.tree.dfs() {
            match &node.t {
                NodeType::None => println!("Type of node {} is None", node.name()),
                NodeType::Exe => println!("Type of node {} is Exe", node.name()),
                NodeType::Condition(opt) => match opt {
                    Some(s) => {
                        if let Some(evaluator) = &self.commands {
                            println!(
                                "Type of node {} is Condition with predicate {}, which evaluates to {}",
                                node.name(),
                                s,
                                evaluator.eval_predicate(s)
                                )
                        }
                    }

                    None => println!(
                        "Type of node {} is Condition with predicate TRUE",
                        node.name()
                    ),
                },
            }
            for (index, value) in node.values().iter().enumerate() {
                println!("value # {} is {}", index, value);
            }
        }
    }
}
