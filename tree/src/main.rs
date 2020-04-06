use std::collections::VecDeque;
use std::fs;

#[derive(Debug)]
struct Node {
    value: String,
    children: Vec<Box<Node>>,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.children == other.children
    }
}

struct Tree {
    root: Box<Node>,
}

struct TreeParser {
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
        stack.push(Node {
            value: String::from("default_root_value"),
            children: vec![],
        });
        for line in data.lines() {
            match line.trim() {
                "{" => stack.push(Node {
                    value: String::from(""),
                    children: vec![],
                }),
                "}" => {
                    let previous_to_last_index = stack.len() - 2;
                    let last = stack.pop()?;
                    stack.get_mut(previous_to_last_index)?.add(last);
                }
                l => {
                    stack.last_mut()?.value = String::from(l);
                }
            }
        }
        let tree = Tree {
            root: Box::new(stack.pop()?),
        };

        Some(tree)
    }
}

impl Tree {
    pub fn print_df(&self) {
        println!("Printing depth first traversal");
        self.root.print_df();
        println!("Done!");
    }

    pub fn print_bf(&self) {
        println!("Printing breadth first traversal");
        let mut used = vec![&self.root];
        let mut queue = VecDeque::new();
        queue.push_back(&self.root);

        while !queue.is_empty() {
            if let Some(node) = queue.pop_front() {
                print!("{} ", node.value);
                for child in &node.children {
                    if !used.contains(&&child) {
                        used.push(&child);
                        queue.push_back(&child);
                    }
                }
            }
        }
        println!("Done!");
    }
}

impl Node {
    pub fn add(&mut self, leaf: Node) {
        self.children.push(Box::new(leaf));
    }

    pub fn print_df(&self) {
        print!("{} ", self.value);
        for node in &self.children {
            node.print_df();
        }
    }
}

fn main() {
    let parser = TreeParser::new("assets/task.txt");
    if let Some(tree) = parser.read_from_file() {
        tree.print_df();
        tree.print_bf();
    }
}
