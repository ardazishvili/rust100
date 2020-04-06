use std::fs;

#[derive(Debug)]
struct Node {
    value: String,
    children: Vec<Box<Node>>,
}

struct Tree {
    id: u32,
    root: Option<Node>,
}

impl Tree {
    pub fn new(id: u32) -> Tree {
        Tree { id: id, root: None }
    }

    pub fn read_from_file(&mut self, filename: &str) -> Option<bool> {
        let data = match fs::read_to_string(filename) {
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
                l => stack.last_mut()?.value = String::from(l),
            }
            println!("{}", line);
        }
        self.root = stack.pop();
        println!("self.root =  {:?}", self.root);

        Some(true)
    }
}

impl Node {
    pub fn add(&mut self, leaf: Node) {
        self.children.push(Box::new(leaf));
    }

    pub fn print_children(&self) {
        for child in &self.children {
            println!("{:#?}", child.value);
        }
    }
}

fn main() {
    let mut tree = Tree::new(0);

    tree.read_from_file("assets/complex.txt");
}
