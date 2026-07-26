use std::fs;
use serde::Deserialize;

use crate::types::Task;

fn ReadData() {
    let content = fs::read_to_string("~/.local/share/task/do.json")
        .expect("Impossible to read the file");

    let TaskList: Vec<Task>;

}
