use crate::parser::TreeParser;

mod iterators;
mod parser;
mod tree;

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
