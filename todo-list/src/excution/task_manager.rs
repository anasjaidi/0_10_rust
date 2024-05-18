use super::{task::Task, task_status::TaskStatus};

pub struct TaskManager {
    tasks: Vec<Task>,
}

impl TaskManager {
    fn new() -> Self {
        Self { tasks: vec![] }
    }

    fn add(&mut self, name: String, description: Option<String>) -> &Task {
        let id = self.tasks.len() as u128 + 1u128;

        let task = Task::new(id, name, description);

        self.tasks.push(task);

        &self.tasks[id as usize - 1]
    }

    fn update(&mut self, id: usize, args: Vec<(String, String)>) -> Option<&Task> {
        let task = self.tasks.get_mut(id)?;
        for (k, v) in args {
            match k.as_str() {
                "--name" => {
                    task.update_name(v);
                }
                "--description" => {
                    task.update_description(v);
                }
                "--status" => {
                    let status = match TaskStatus::try_from(v) {
                        Err(_) => return None,
                        Ok(data) => data,
                    };
                    task.update_status(status);
                }
                _ => {}
            };
        }
        Some(task)
    }

    fn remove(&mut self, id: usize) -> Option<Task> {
        Some(self.tasks.remove(id))
    }
}
