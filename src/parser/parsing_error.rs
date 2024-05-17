pub enum ParsingError<'a> {
    EmptyLine,
    UnexpectedToken(&'a String),
    InvalidModifier(&'a String),
    InvalidCommand(&'a String),
    InvalidArgument(&'a String),
}

impl<'a> ParsingError<'a> {
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
