use crate::iterators::{Node, NodeType};
use crate::tree::Tree;
use regex::Regex;
use std::fs;

#[derive(PartialEq)]
pub enum ParseType {
    Commands,
    Scenario,
}

pub struct TreeParser {
    filename: String,
    t: ParseType,
}

impl TreeParser {
    pub fn new(filename: &str, t: ParseType) -> TreeParser {
        TreeParser {
            filename: String::from(filename),
            t,
        }
    }

    pub fn read(&self) -> Option<Tree> {
        let data = match fs::read_to_string(&self.filename) {
            Ok(s) => s,
            Err(_) => panic!("Can't read file"),
        };
        let mut stack: Vec<Node> = vec![];
        stack.push(Node::new(
            self.filename.clone(),
            NodeType::None,
            vec![],
            vec![],
        ));
        for line in data.lines() {
            match line.trim() {
                "{" => continue,
                " " => continue,
                "}" => {
                    let previous_to_last_index = stack.len() - 2;
                    let last = stack.pop()?;
                    stack.get_mut(previous_to_last_index)?.add(last);
                }
                l => {
                    if l.starts_with("Команда") || l.starts_with("Сценарий") {
                        stack.push(Node::new(String::from(l), NodeType::None, vec![], vec![]));
                    } else if l.starts_with("Если") {
                        let re = Regex::new(r"Если (.*)").unwrap();
                        if let Some(predicate) = re.captures(l).unwrap().get(1) {
                            stack.push(Node::new(
                                String::from(l),
                                NodeType::Condition(Some(String::from(predicate.as_str()))),
                                vec![],
                                vec![],
                            ));
                        }
                    } else if l.starts_with("Иначе") {
                        stack.push(Node::new(
                            String::from(l),
                            NodeType::Condition(None),
                            vec![],
                            vec![],
                        ));
                    } else {
                        if self.t == ParseType::Commands {
                            stack.last_mut()?.add_value(l);
                        } else {
                            let last_index = stack.len() - 1;
                            stack.get_mut(last_index)?.add(Node::new(
                                String::from(l),
                                NodeType::Exe,
                                vec![],
                                vec![],
                            ));
                        }
                    }
                }
            }
        }
        let tree = Tree::new(Box::new(stack.pop()?));

        Some(tree)
    }
}
