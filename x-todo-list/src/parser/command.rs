use super::Argument;

#[derive(Debug)]
pub struct Command {
    pub cmd: String,
    pub modifiers: Vec<String>,
    pub args: Vec<Argument>,
}
