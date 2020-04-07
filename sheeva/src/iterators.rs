pub use std::collections::VecDeque;

#[derive(Debug)]
pub struct Node {
    name: String,
    values: Vec<String>,
    children: Vec<Box<Node>>,
}

impl Node {
    pub fn new(name: String, values: Vec<String>, children: Vec<Box<Node>>) -> Node {
        Node {
            name,
            values,
            children,
        }
    }

    pub fn add_value(&mut self, s: &str) {
        self.values.push(String::from(s));
    }

    pub fn add(&mut self, leaf: Node) {
        self.children.push(Box::new(leaf));
    }

    pub fn values(&self) -> &Vec<String> {
        &self.values
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn children(&self) -> &Vec<Box<Node>> {
        &self.children
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.values == other.values && self.children == other.children && self.name == other.name
    }
}

pub struct DFSIterator<'a> {
    queue: VecDeque<&'a Box<Node>>,
}

impl<'a> DFSIterator<'a> {
    pub fn new(queue: VecDeque<&'a Box<Node>>) -> DFSIterator<'a> {
        DFSIterator { queue }
    }
}

pub struct BFSIterator<'a> {
    used: Vec<&'a Box<Node>>,
    queue: VecDeque<&'a Box<Node>>,
}

impl<'a> BFSIterator<'a> {
    pub fn new(used: Vec<&'a Box<Node>>, queue: VecDeque<&'a Box<Node>>) -> BFSIterator<'a> {
        BFSIterator { used, queue }
    }
}

impl<'a> Iterator for DFSIterator<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<&'a Node> {
        match self.queue.pop_front() {
            None => None,
            Some(node) => {
                for child in node.children.iter().rev() {
                    self.queue.push_front(&child);
                }
                Some(node)
            }
        }
    }
}

impl<'a> Iterator for BFSIterator<'a> {
    type Item = &'a Node;
    fn next(&mut self) -> Option<&'a Node> {
        match self.queue.pop_front() {
            None => None,
            Some(node) => {
                for child in &node.children {
                    if !self.used.contains(&&child) {
                        self.used.push(&child);
                        self.queue.push_back(&child);
                    }
                }
                Some(node)
            }
        }
    }
}
