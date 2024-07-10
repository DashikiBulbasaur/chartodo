use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Tasks {
    pub todo: Vec<Task>,
    pub done: Vec<Task>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Task {
    pub task: String,
    pub date: Option<String>,
    pub time: Option<String>,
    pub repeat_number: Option<usize>,
    pub repeat_unit: Option<String>,
    pub repeat_done: Option<bool>,
}