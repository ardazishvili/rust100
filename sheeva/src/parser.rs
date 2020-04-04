use crate::command::Command;
use std::fs;

pub struct Parser {
    commands: Vec<Command>,
}

impl Parser {
    pub fn new() -> Parser {
        Parser { commands: vec![] }
    }

    pub fn read(&mut self, filename: &str) {
        let text = match fs::read_to_string(filename) {
            Ok(r) => r,
            Err(_) => panic!("Can't read specified file"),
        };

        let (mut start, mut name) = (false, String::from(""));
        let lines = text.lines();
        for line in lines {
            match line {
                "{" => start = true,
                "}" => start = false,

                line => match start {
                    true => self
                        .commands
                        .push(Command::new(name.clone(), String::from(line))),
                    false => name = String::from(line),
                },
            }
        }
    }

    pub fn commands(&self) -> &Vec<Command> {
        &self.commands
    }
}
