use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq)]
pub enum Status {
    Pending,
    InProgress,
    Done
}

pub struct Group {
    name: String,
    status: Status,
    tasks: Vec<Task>
}

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub name: String,
    pub status: Status,
}
