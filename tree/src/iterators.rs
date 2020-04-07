pub use std::collections::VecDeque;

#[derive(Debug)]
pub struct Node {
    value: String,
    children: Vec<Box<Node>>,
}

impl Node {
    pub fn new(value: String, children: Vec<Box<Node>>) -> Node {
        Node { value, children }
    }

    pub fn set_value(&mut self, s: &str) {
        self.value = String::from(s);
    }

    pub fn add(&mut self, leaf: Node) {
        self.children.push(Box::new(leaf));
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.children == other.children
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
    }
}
