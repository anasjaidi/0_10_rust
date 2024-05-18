use super::task_status::TaskStatus;

pub struct Task {
    pub id: u128,
    pub name: String,
    pub description: Option<String>,
    pub status: TaskStatus,
}

impl Task {
    pub fn new(id: u128, name: String, description: Option<String>) -> Self {
        Self {
            id,
            name,
            description,
            status: TaskStatus::Todo,
        }
    }

    pub fn update_name(&mut self, name: String) -> &Self {
        self.name = name;
        self
    }

    pub fn update_description(&mut self, description: String) -> &Self {
        self.description = Some(description);
        self
    }

    pub fn update_status(&mut self, status: TaskStatus) -> &Self {
        self.status = status;
        self
    }
}
