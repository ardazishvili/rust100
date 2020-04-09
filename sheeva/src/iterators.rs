use crate::command::Expressions;
pub use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
pub enum NodeType {
    None,
    Exe,
    Condition(Option<String>),
}

#[derive(Debug)]
pub struct Node {
    name: String,
    values: Vec<String>,
    children: Vec<Box<Node>>,
    pub t: NodeType,
}

impl Node {
    pub fn new(name: String, t: NodeType, values: Vec<String>, children: Vec<Box<Node>>) -> Node {
        Node {
            name,
            t,
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

pub struct ConditionIterator<'a> {
    queue: VecDeque<&'a Box<Node>>,
    evaluator: &'a Expressions,
    if_was_true: bool,
}

impl<'a> ConditionIterator<'a> {
    pub fn new(
        queue: VecDeque<&'a Box<Node>>,
        evaluator: &'a Expressions,
    ) -> ConditionIterator<'a> {
        ConditionIterator {
            queue,
            evaluator,
            if_was_true: false,
        }
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

impl<'a> Iterator for ConditionIterator<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<&'a Node> {
        match self.queue.pop_front() {
            None => None,
            Some(node) => match &node.t {
                NodeType::None => {
                    for child in node.children.iter().rev() {
                        self.queue.push_front(&child);
                    }
                    Some(node)
                }
                NodeType::Exe => {
                    for child in node.children.iter().rev() {
                        self.queue.push_front(&child);
                    }
                    Some(node)
                }
                NodeType::Condition(opt) => match opt {
                    None => {
                        // if the 'if' branch was true at the previous stage DO NOT yield value at
                        // current 'else' stage
                        println!("Node {}, if_was_true: {}", node.name(), self.if_was_true);
                        if !self.if_was_true {
                            for child in node.children.iter().rev() {
                                self.queue.push_front(&child);
                            }
                            self.if_was_true = false;
                            Some(node)
                        //else: ignore
                        } else {
                            self.next()
                        }
                    }
                    Some(predicate) => {
                        if self.evaluator.eval_predicate(predicate) {
                            for child in node.children.iter().rev() {
                                self.queue.push_front(&child);
                            }
                            // mark the state of iterator: 'else' branch will not yield at the 'next()'
                            // step
                            println!("Setting if_was_true");
                            self.if_was_true = true;
                            Some(node)
                        } else {
                            self.if_was_true = false;
                            self.next()
                        }
                    }
                },
            },
        }
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
