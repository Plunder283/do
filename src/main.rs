use std::env;
use std::process::Command;

mod types;
mod data;

fn launch_editor(file: &str) {
    let status = Command::new("vim")
            .arg(format!("~/.local/share/task/{file}"))
            .status()
            .expect("Impossible to run vim");

    if status.success() {
        println!("Your notes had been saved!");
    } else {
        println!("An error occurred while writing your notes");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let argc = args.len();

    if argc == 1 {
        data::ListTasks();
        return;
    }

    if args[1] == "note" || args[1] == "draft" {
        launch_editor("draft.txt");
        return;
    } else if args[1] == "add" || args[1] == "-a" || args[1] == "a" {  
        // Error Handling
        if argc <= 2 {
            println!("Usage: do add [task1] [task2] ...");
            return;
        }

        let mut i = 2;
        while i < argc {
            data::AddTask(args[i].clone()).unwrap();
            i = i + 1;
        }
    }
}
