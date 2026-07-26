use std::env;
use std::process::Command;

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
    
    if args.len() == 1 {
        launch_editor(todo);
        return;
    }

    if args[1] == "note" || args[1] == "draft" {
        launch_editor("draft.txt");
        return;
    } else if args[1] == "add" {

    }
}
