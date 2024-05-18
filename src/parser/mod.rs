mod argument;
mod command;
mod parsing_error;

pub use argument::Argument;
pub use command::Command;
pub use parsing_error::ParsingError;

pub fn parse_args(
    tokens: &[String],
    accept_args: bool,
    modofiers: Vec<&str>,
    possible_args: Vec<&str>,
) -> Result<(Vec<Argument>, Vec<String>), ParsingError> {
    if !accept_args && !tokens.is_empty() {
        return Err(ParsingError::UnexpectedToken(
            tokens.first().unwrap().clone(),
        ));
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
            Some(Argument::new(
                key.to_string(),
                value.chars().filter(|&c| c != '\'' && c != '\"').collect(),
            ))
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
            return Err(ParsingError::UnexpectedToken(t.clone()));
        }
    }

    Ok((args, modifiers))
}

pub fn split_tokens(line: &str) -> Result<Vec<String>, ParsingError> {
    let mut tokens = vec![];
    let mut p1 = 0;
    let mut quote = false;
    let is_quote = |c| c == '\"' || c == '\'';
    let is_white_space = |c: char| c.is_whitespace();
    for (i, c) in line.chars().enumerate() {
        if is_quote(c) {
            quote = !quote;
            if quote {
                continue;
            }
        } else if is_white_space(c) {
            if quote {
                continue;
            }
        } else {
            continue;
        }
        let sub = line[p1..i + 1].trim().to_string();
        if !sub.is_empty() {
            tokens.push(sub)
        };
        p1 = i + 1;
    }

    if quote {
        Err(ParsingError::EmptyLine)
    } else {
        Ok(tokens)
    }
}

pub fn parse_line(line: &str) -> Result<Command, ParsingError> {
    if line.is_empty() {
        return Err(ParsingError::EmptyLine);
    }
    let mut tokens: Vec<String> = match split_tokens(line) {
        Err(err) => return Err(err),
        Ok(t) => t,
    };
    let cmd = tokens.remove(0);
    match cmd.as_str() {
        "show" => {
            parse_args(&tokens, true, vec!["--all"], vec!["--id"]).map(|(args, modifiers)| {
                Command {
                    cmd,
                    modifiers,
                    args,
                }
            })
        }
        "add" => parse_args(
            &tokens,
            true,
            vec![],
            vec!["--name", "--description", "--sub-tasks"],
        )
        .map(|(args, modifiers)| Command {
            cmd,
            modifiers,
            args,
        }),
        "remove" => {
            parse_args(&tokens, true, vec!["--all"], vec!["--id"]).map(|(args, modifiers)| {
                Command {
                    cmd,
                    modifiers,
                    args,
                }
            })
        }
        "exit" => parse_args(&tokens, false, vec![], vec![]).map(|(args, modifiers)| Command {
            cmd,
            modifiers,
            args,
        }),
        _ => Err(ParsingError::InvalidCommand(cmd)),
    }
}
