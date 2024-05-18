use self::{task::Task, task_status::TaskStatus};

mod task;
mod task_manager;
mod task_status;

pub use task_manager::TaskManager;

pub enum RunTimeError {}

fn excute_argument(
    task_manager: &TaskManager,
    cmd: String,
    modidifers: Vec<String>,
    args: Vec<(String, String)>,
) -> Result<(), RunTimeError> {
    match cmd.as_str() {
        "add" => {}
        _ => {}
    }
    Ok(())
}
