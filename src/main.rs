fn pad_string(input: &String, size: usize) -> String {
    if input.len() > size {
        format!("{}...", &input[..size - 3])
    } else {
        format!("{:<1$}", input, size)
    }
}

#[derive(Debug, Clone)]
enum TodoStatus {
    Backlog,
    Todo,
    OnProgress,
    Blocked,
    Done,
}

impl TodoStatus {
    fn format(&self) -> String {
        match self {
            Self::Backlog => "backlog".to_owned(),
            Self::Todo => "todo".to_owned(),
            Self::OnProgress => "on progress".to_owned(),
            Self::Blocked => "blocked".to_owned(),
            Self::Done => "done".to_owned(),
        }
    }

    fn from_string(status_str: &String) -> Option<Self> {
        match status_str.to_lowercase().as_str() {
            "backlog" => Some(Self::Backlog),
            "todo" => Some(Self::Todo),
            "onprogress" => Some(Self::OnProgress),
            "blocked" => Some(Self::Blocked),
            "done" => Some(Self::Done),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
struct Todo {
    id: u32,
    name: String,
    description: Option<String>,
    status: TodoStatus,
    dependencies: Vec<u32>,
}

impl Todo {
    fn new(
        id: u32,
        name: String,
        status: TodoStatus,
        dependecies: Option<Vec<u32>>,
        description: Option<String>,
    ) -> Self {
        Self {
            id,
            name,
            status,
            dependencies: dependecies.unwrap_or_default(),
            description,
        }
    }

    fn update(
        &mut self,
        name: Option<&String>,
        status: Option<&TodoStatus>,
        description: Option<&String>,
    ) {
        if let Some(name) = name {
            self.name = name.to_owned();
        }
        if let Some(status) = status {
            self.status = status.to_owned();
        }
        if description.is_some() {
            self.description = match description {
                Some(d) => Some(d.to_owned()),
                _ => None,
            };
        }
    }

    fn add_dependesies(&mut self, dp: &Vec<u32>) {
        dp.iter().for_each(|x| {
            if !self.dependencies.contains(x) {
                self.dependencies.push(*x);
            }
        });
    }

    fn remove_dependencies(&mut self, dp: &Vec<u32>) {
        self.dependencies = self
            .dependencies
            .iter()
            .filter(|x| dp.contains(x))
            .cloned()
            .collect();
    }

    fn display(&self, sep: char, pad_size: usize) {
        let description = self
            .description
            .as_ref()
            .map_or("None".to_owned(), |x| x.clone());

        let print_fields = |f: &String, s: Option<char>| -> String {
            if s.is_some() {
                format!("{}{}", pad_string(f, pad_size), s.unwrap())
            } else {
                format!("{}", pad_string(f, pad_size))
            }
        };

        print!("{}", print_fields(&self.id.to_string(), Some(sep)));
        print!("{}", print_fields(&self.name, Some(sep)));
        print!("{}", print_fields(&self.status.format(), Some(sep)));
        print!("{}", print_fields(&description, Some(sep)));
        println!(
            "{}",
            print_fields(&format!("{:?}", self.dependencies), None)
        );
    }
}

struct TasksManager {
    tasks: Vec<Todo>,
}

impl TasksManager {
    fn new() -> Self {
        Self { tasks: vec![] }
    }

    fn add(&mut self, t: &Todo) {
        self.tasks.push(t.clone());
    }

    fn remove(&mut self, id: usize) -> Option<Todo> {
        if let Some(pos) = self.tasks.iter().position(|x| x.id == id as u32) {
            Some(self.tasks.remove(pos))
        } else {
            None
        }
    }

    fn update() -> Option<Todo> {
        None
    }
}

struct Command {
    cmd: String,
    modifiers: Vec<String>,
    args: Vec<(String, String)>,
}

enum ParsingErrors {
    ArgumentAlreadyExists,
    ModifierAlreadyExists,
    InvalidArgument,
    InvalidModifier,
    InvalidToken,
    InvalidCommand,
}

impl Command {
    fn new(cmd: String) -> Self {
        Self {
            cmd,
            modifiers: vec![],
            args: vec![],
        }
    }

    fn add_argument(&mut self, k: &str, v: &str) -> Result<(), ParsingErrors> {
        if self.args.iter().any(|(_k, _)| _k == k) {
            Err(ParsingErrors::ArgumentAlreadyExists)
        } else {
            self.args.push((k.to_owned(), v.to_owned()));
            Ok(())
        }
    }
    fn add_modifier(&mut self, modifier: &str) {
        self.modifiers.push(modifier.to_owned());
    }
}

fn parse_command(cmd: String) -> Result<(String, Vec<String>), String> {
    let mut tokens = cmd.split(' ');
    Ok((String::new(), vec![]))
}

fn main() {}
