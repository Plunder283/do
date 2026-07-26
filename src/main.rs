use std::env;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() == 1 {
        return; // Faire plus tard
    }

    if args[1] == "note" || args[1] == "" {
        let status = Command::new("vim")
            .arg("~/.local/share/task/note.txt")
            .status()
            .expect("Impossible to run vim");

        if status.success() {
            println!("Your notes had been saved!");
        } else {
            println!("An error occurred while writing your notes");
        }
    }
}
