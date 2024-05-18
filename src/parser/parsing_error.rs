#[derive(Debug)]
pub enum ParsingError {
    EmptyLine,
    UnexpectedToken(String),
    InvalidModifier(String),
    InvalidCommand(String),
    InvalidArgument(String),
}

impl ParsingError {
    pub fn handle(&self) {
        match self {
            ParsingError::EmptyLine => eprintln!("Error: Empty line"),
            ParsingError::UnexpectedToken(token) => eprintln!("Unexpected token: {}", token),
            ParsingError::InvalidModifier(modifier) => eprintln!("Invalid modifier: {}", modifier),
            ParsingError::InvalidCommand(cmd) => eprintln!("Invalid command: {}", cmd),
            ParsingError::InvalidArgument(arg) => eprintln!("Invalid argument: {}", arg),
        }
    }
}
