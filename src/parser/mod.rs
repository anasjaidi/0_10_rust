mod argument;
mod command;
mod parsing_error;

pub use argument::Argument;
pub use command::Command;
pub use parsing_error::ParsingError;

pub fn parse_args<'a>(
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
                return None;
            }
            let value = parts[1].trim();
            // Create and return the Argument.
            Some(Argument::new(key.to_string(), value.to_string()))
        } else {
            // If the input string doesn't contain '=', it's not a valid argument.
            None
        }
    };
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

pub fn parse_line(line: &String) -> Result<Command, ParsingError> {
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
