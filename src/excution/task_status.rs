pub enum TaskStatus {
    Todo = 1,
    Doing = 2,
    Done = 3,
}

impl ToString for TaskStatus {
    fn to_string(&self) -> String {
        match self {
            TaskStatus::Todo => "todo".to_string(),
            TaskStatus::Doing => "doing".to_string(),
            TaskStatus::Done => "done".to_string(),
        }
    }
}

impl TryFrom<u8> for TaskStatus {
    type Error = String;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Todo),
            2 => Ok(Self::Doing),
            3 => Ok(Self::Done),
            _ => Err(String::new()),
        }
    }
}

impl TryFrom<String> for TaskStatus {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "todo" => Ok(Self::Todo),
            "doing" => Ok(Self::Doing),
            "done" => Ok(Self::Done),
            _ => Err(String::new()),
        }
    }
}
