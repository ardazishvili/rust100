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

struct DFSIterator<'a> {
    queue: VecDeque<&'a Box<Node>>,
}

struct BFSIterator<'a> {
    used: Vec<&'a Box<Node>>,
    queue: VecDeque<&'a Box<Node>>,
}

impl<'a> Iterator for DFSIterator<'a> {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        match self.queue.pop_front() {
            None => None,
            Some(node) => {
                for child in node.children.iter().rev() {
                    self.queue.push_front(&child);
                }
                Some(node.value.clone())
            }
        }
    }
}

impl<'a> Iterator for BFSIterator<'a> {
    type Item = String;
    fn next(&mut self) -> Option<String> {
        match self.queue.pop_front() {
            None => None,
            Some(node) => {
                for child in &node.children {
                    if !self.used.contains(&&child) {
                        self.used.push(&child);
                        self.queue.push_back(&child);
                    }
                }
                Some(node.value.clone())
            }
        }

        // while !queue.is_empty() {
        //     if let Some(node) = queue.pop_front() {
        //         print!("{} ", node.value);
        //         for child in &node.children {
        //             if !used.contains(&&child) {
        //                 used.push(&child);
        //                 queue.push_back(&child);
        //             }
        //         }
        //     }
        // }
    }
}

impl Tree {
    // pub fn print_df(&self) {

    //     let mut queue = VecDeque::new();
    //     queue.push_front(&self.root);

    //     while !queue.is_empty() {
    //         if let Some(node) = queue.pop_front() {
    //             print!("{} ", node.value);
    //             for child in node.children.iter().rev() {
    //                 queue.push_front(&child);
    //             }
    //         }
    //     }

    //     println!("Done!");
    // }

    // pub fn print_bf(&self) {
    //     let mut used = vec![&self.root];
    //     let mut queue = VecDeque::new();
    //     queue.push_back(&self.root);

    //     while !queue.is_empty() {
    //         if let Some(node) = queue.pop_front() {
    //             print!("{} ", node.value);
    //             for child in &node.children {
    //                 if !used.contains(&&child) {
    //                     used.push(&child);
    //                     queue.push_back(&child);
    //                 }
    //             }
    //         }
    //     }
    //     println!("Done!");
    // }

    fn dfs<'a>(&'a self) -> DFSIterator<'a> {
        let mut q = VecDeque::new();
        q.push_front(&self.root);
        DFSIterator { queue: q }
    }

    fn bfs<'a>(&'a self) -> BFSIterator<'a> {
        let mut q = VecDeque::new();
        q.push_front(&self.root);
        let mut u = vec![&self.root];
        BFSIterator { used: u, queue: q }
    }
}

impl Node {
    pub fn add(&mut self, leaf: Node) {
        self.children.push(Box::new(leaf));
    }
}

fn main() {
    let parser = TreeParser::new("assets/task.txt");
    if let Some(tree) = parser.read_from_file() {
        println!("Printing depth first traversal");
        for i in tree.dfs() {
            print!("{} ", i);
        }
        println!("\nPrinting breadth first traversal");
        for i in tree.bfs() {
            print!("{} ", i);
        }
    }
}
