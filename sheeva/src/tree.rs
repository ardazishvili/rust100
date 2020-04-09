use crate::command::Expressions;
use crate::iterators::*;

pub struct Tree {
    root: Box<Node>,
}

impl Tree {
    pub fn new(root: Box<Node>) -> Tree {
        Tree { root }
    }

    pub fn dfs<'a>(&'a self) -> DFSIterator<'a> {
        let mut q = VecDeque::new();
        q.push_front(&self.root);
        DFSIterator::new(q)
    }

    pub fn bfs<'a>(&'a self) -> BFSIterator<'a> {
        let mut q = VecDeque::new();
        q.push_front(&self.root);
        BFSIterator::new(vec![&self.root], q)
    }

    pub fn cond_iter<'a>(&'a self, evaluator: &'a Expressions) -> ConditionIterator<'a> {
        let mut q = VecDeque::new();
        q.push_front(&self.root);
        ConditionIterator::new(q, evaluator)
    }
}
