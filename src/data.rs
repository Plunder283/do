use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;

use crate::types::{Task, Status};

fn get_path() -> PathBuf {
    let mut path = dirs::home_dir().expect("Impossible de trouver le dossier home");
    path.push(".local/share/task/do.json");
    path
}

fn ReadData() -> Vec<Task> {
    let path = get_path();

    let content = match fs::read_to_string(&path) {
        Ok(c) => c,
        Err(e) if e.kind() == ErrorKind::NotFound => {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).expect("Impossible de créer le dossier");
            }
            fs::write(&path, "[]").expect("Impossible de créer le fichier");
            "[]".to_string()
        }
        Err(e) => panic!("Impossible de lire le fichier : {e}"),
    };

    serde_json::from_str(&content).expect("Invalid JSON")
}

fn WriteData(task_list: &Vec<Task>) -> Result<(), String> {
    let path = get_path();
    let json = serde_json::to_string_pretty(task_list)
        .map_err(|e| format!("JSON serialization error : {e}"))?;
    fs::write(&path, json)
        .map_err(|e| format!("Impossible to write the file : {e}"))
}

pub fn AddTask(task_name: String) -> Result<(), String> {
    let mut task_list: Vec<Task> = ReadData();
    for task in &task_list {
        if task.name == task_name {
            return Err(format!("Task: {task_name} already exist"))
        }
    }
    let new_task: Task = Task {
        name: task_name,
        status: Status::Pending
    };
    task_list.push(new_task);
    WriteData(&task_list).unwrap();
    Ok(())
}

pub fn DoTask(task_name: String) -> Result<(), String> {
    let task_list: Vec<Task> = ReadData();
    for mut task in task_list {
        if task.name == task_name {
            task.status = Status::Done;
            return Ok(());
        }
    }
    return Err(format!("Task: {task_name} does not exist"));
}

pub fn clean_tasks() {
    let mut task_list: Vec<Task> = ReadData();
    task_list.retain(|task| task.status != Status::Done);
    WriteData(&task_list).unwrap();
}

pub fn ListTasks() {
    let task_list: Vec<Task> = ReadData();
    for task in task_list {
        match task.status {
            Status::Pending    => println!("❌ {0}", task.name),
            Status::InProgress => println!("⏳ {0}", task.name),
            Status::Done       => println!("✅ {0}", task.name)
        }
    }
}
