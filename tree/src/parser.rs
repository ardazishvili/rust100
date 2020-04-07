use crate::iterators::Node;
use crate::tree::Tree;
use std::fs;

// mod iterators;
// mod tree;

pub struct TreeParser {
    filename: String,
}

impl TreeParser {
    pub fn new(filename: &str) -> TreeParser {
        TreeParser {
            filename: String::from(filename),
        }
    }

    pub fn read_from_file(&self) -> Option<Tree> {
        let data = match fs::read_to_string(&self.filename) {
            Ok(s) => s,
            Err(_) => panic!("Can't read file"),
        };
        let mut stack: Vec<Node> = vec![];
        stack.push(Node::new(String::from("default_root_value"), vec![]));
        for line in data.lines() {
            match line.trim() {
                "{" => stack.push(Node::new(String::from(""), vec![])),
                "}" => {
                    let previous_to_last_index = stack.len() - 2;
                    let last = stack.pop()?;
                    stack.get_mut(previous_to_last_index)?.add(last);
                }
                l => {
                    stack.last_mut()?.set_value(l);
                }
            }
        }
        let tree = Tree::new(Box::new(stack.pop()?));

        Some(tree)
    }
}
