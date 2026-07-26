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

pub struct Task {
    name: String,
    status: String,
}
