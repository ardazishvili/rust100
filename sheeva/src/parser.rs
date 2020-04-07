use crate::iterators::Node;
use crate::tree::Tree;
use std::fs;

pub struct TreeParser {
    filename: String,
}

impl TreeParser {
    pub fn new(filename: &str) -> TreeParser {
        TreeParser {
            filename: String::from(filename),
        }
    }

    pub fn read(&self) -> Option<Tree> {
        let data = match fs::read_to_string(&self.filename) {
            Ok(s) => s,
            Err(_) => panic!("Can't read file"),
        };
        let mut stack: Vec<Node> = vec![];
        stack.push(Node::new(self.filename.clone(), vec![], vec![]));
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
                    if l.starts_with("Команда")
                        || l.starts_with("Сценарий")
                        || l.starts_with("Если")
                    {
                        stack.push(Node::new(String::from(l), vec![], vec![]));
                    } else {
                        stack.last_mut()?.add_value(l);
                    }
                }
            }
        }
        let tree = Tree::new(Box::new(stack.pop()?));

        Some(tree)
    }
}
