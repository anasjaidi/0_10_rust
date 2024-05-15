fn pad_string(input: String, size: usize) -> String {
    if input.len() > size {
        format!("{}...", &input[..size - 3])
    } else {
        format!("{:<1$}", input, size)
    }
}

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

    fn display(&self, sep: char, pad_size: usize) {}
}

struct TasksManager {
    tasks: Vec<Todo>,
}

impl TasksManager {}

fn main() {
    println!("Hello, world!");
}
