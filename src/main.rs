use std::{
    io::{self, Write},
    process,
};

struct Argument(String, String);

impl Argument {
    fn new(k: String, v: String) -> Self {
        Self(k, v)
    }

    fn key(&self) -> &String {
        &self.0
    }

    fn value(&self) -> &String {
        &self.1
    }
}

struct Command {
    cmd: String,
    modifiers: Vec<String>,
    args: Vec<Argument>,
}

enum ParsingError<'a> {
    EmptyLine,
    UnexpectedToken(&'a String),
    InvalidModifier(&'a String),
    InvalidCommand(&'a String),
    InvalidArgument(&'a String),
}

impl ParsingError<'_> {
    fn handle(&self) {
        match self {
            _ => {}
        };
    }
}

fn parse_exit(tokens: &Vec<String>) -> Result<(), ParsingError> {
    if tokens.len() > 0 {
        Err(ParsingError::UnexpectedToken(tokens.get(0).unwrap()))
    } else {
        Ok(())
    }
}

fn parse_show(tokens: &Vec<String>) -> Result<(), ParsingError> {}
fn parse_add(tokens: &Vec<String>) -> Result<(), ParsingError> {}
fn parse_remove(tokens: &Vec<String>) -> Result<(), ParsingError> {}
fn parse_update(tokens: &Vec<String>) -> Result<(), ParsingError> {}

fn parse_line(line: &String) -> Result<Command, ParsingError> {
    if line.len() == 0 {
        return Err(ParsingError::EmptyLine);
    }
    let mut tokens: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
    let cmd = tokens.remove(0);
    match cmd.as_str() {
        "show" => {}
        "add" => {}
        "remove" => {}
        "exit" => {}
        _ => {}
    };
    Err(ParsingError::EmptyLine)
}

fn main() {
    let mut line = String::new();
    loop {
        print!("todo-app #> ");
        io::stdout().flush().expect("error while fulshing");
        std::io::stdin()
            .read_line(&mut line)
            .map_err(|err| {
                eprintln!("{}", err);
                process::exit(1)
            })
            .ok();
        println!("{}", line);
        match parse_line(&line) {
            Err(err) => {
                err.handle();
                line.clear();
                continue;
            }
            Ok(cmd) => {}
        };
        line.clear();
        println!("line:{}", line);
    }
}
