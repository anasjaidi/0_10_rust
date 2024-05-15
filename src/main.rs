fn pad_string(input: &str, size: usize) -> String {
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

    fn from_string(status_str: String) -> Option<Self> {
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
        name: Option<String>,
        status: Option<TodoStatus>,
        description: Option<String>,
    ) {
        if let Some(name) = name {
            self.name = name;
        }
        if let Some(status) = status {
            self.status = status;
        }
        if description.is_some() {
            self.description = description;
        }
    }

    fn add_dependesies(&mut self, dp: Vec<u32>) {
        dp.iter().for_each(|x| {
            if !self.dependencies.contains(x) {
                self.dependencies.push(*x);
            }
        });
    }

    fn remove_dependencies(&mut self, dp: Vec<u32>) {
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

        let print_fields = |f: &str, s: Option<char>| -> String {
            if s.is_some() {
                format!("{}{}", pad_string(f, pad_size), s.unwrap())
            } else {
                format!("{}", pad_string(f, pad_size))
            }
        };

        print!("{}", print_fields(self.id.to_string().as_str(), Some(sep)));
        print!("{}", print_fields(self.name.as_str(), Some(sep)));
        print!("{}", print_fields(self.status.format().as_str(), Some(sep)));
        print!("{}", print_fields(description.as_str(), Some(sep)));
        println!(
            "{}",
            print_fields(format!("{:?}", self.dependencies).as_str(), None)
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

fn parse_command(cmd: String) -> Result<Vec<String>, String> {
    let command: Vec<String> = cmd.split(' ').into_iter().map(|s| s.to_owned()).collect();
    match command[0].as_str() {
        "show" => {
            let possible_args = ["--all", "--id"];
            let args = (&command[1..]).to_vec();
            let mut err = false;
            (&args).iter().for_each(|x| {
                err = true;
            });
            Ok(args)
        }
        "add" => Ok(vec![]),
        "remove" => Ok(vec![]),
        "update" => Ok(vec![]),
        _ => Err(String::new() + "Failed to handle command"),
    }
}

fn handle_command(cmd: Vec<String>) -> Result<Todo, String> {
    Err(String::new() + "Failed to handle command")
}

fn main() {}
