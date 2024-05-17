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

impl<'a> ParsingError<'a> {
    fn handle(&self) {
        match self {
            ParsingError::EmptyLine => eprintln!("Error: Empty line"),
            ParsingError::UnexpectedToken(token) => eprintln!("Unexpected token: {}", token),
            ParsingError::InvalidModifier(modifier) => eprintln!("Invalid modifier: {}", modifier),
            ParsingError::InvalidCommand(cmd) => eprintln!("Invalid command: {}", cmd),
            ParsingError::InvalidArgument(arg) => eprintln!("Invalid argument: {}", arg),
        }
    }
}

fn parse_args<'a>(
    tokens: &'a Vec<String>,
    accept_args: bool,
    modofiers: Vec<&str>,
    possible_args: Vec<&str>,
) -> Result<(Vec<Argument>, Vec<String>), ParsingError<'a>> {
    if !accept_args && !tokens.is_empty() {
        return Err(ParsingError::UnexpectedToken(&tokens[0]));
    };

    let is_argument = |s: &String| -> Option<Argument> {
        // Split the input string into key and value parts on the first '=' character.
        let parts: Vec<&str> = s.splitn(2, '=').collect();
        // Ensure that we have exactly two parts (key and value).
        if parts.len() == 2 {
            // Trim whitespace from both key and value.
            let key = parts[0].trim();
            if !possible_args.contains(&key) {
                return None
            }
            let value = parts[1].trim();
            // Create and return the Argument.
            Some(Argument::new(key.to_string(), value.to_string()))
        } else {
            // If the input string doesn't contain '=', it's not a valid argument.
            None
        }
    }
    let is_modifier = |s: &String| modofiers.contains(&s.as_str());

    let mut modifiers = vec![];
    let mut args = vec![];

    for t in tokens.iter() {
        if let Some(arg) = is_argument(t) {
            args.push(arg);
        } else if is_modifier(t) {
            modifiers.push(t.to_owned());
        } else {
            return Err(ParsingError::UnexpectedToken(t));
        }
    }

    Ok((args, modifiers))
}

fn parse_exit(tokens: &Vec<String>) -> Result<(), ParsingError> {
    if tokens.len() > 0 {
        Err(ParsingError::UnexpectedToken(tokens.get(0).unwrap()))
    } else {
        Ok(())
    }
}
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
