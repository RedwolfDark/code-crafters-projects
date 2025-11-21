use std::env::var;
use std::fs::{self};
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

const EXIT_COMMAND: &str = "exit";
const EXECUTABLE_PERMISSION: u32 = 0o111;
const BUILTIN_COMMANDS: [&str; 4] = ["echo", "cd", "exit", "type"];

fn handle_command_not_found(command: &str, context: Option<&str>) {
    println!("{}: {}not found", command, context.unwrap_or(""));
}

fn handle_command(command: &str, args: Vec<&str>) {
    // handle type command
    if command == "type" {
        for arg in &args {
            if BUILTIN_COMMANDS.contains(&arg) {
                println!("{} is a shell builtin", *arg);
            } else if check_command_in_path(&arg).0 {
                println!("{} is {}", *arg, check_command_in_path(&arg).1);
            } else {
                handle_command_not_found(*arg, None);
            }
        }
    }

    // handle built-in commands
    if command == "echo" {
        println!("{}", args.join(" "));
    }

    handle_command_not_found(command, Some("command "));
}

fn handle_inputs() {
    let mut command = String::new();

    io::stdin().read_line(&mut command).unwrap();

    let command = command.trim().to_string();

    let mut command_and_args = command.split_whitespace();
    let command = command_and_args.next().unwrap();
    let args: Vec<&str> = command_and_args.collect();

    match command.contains(EXIT_COMMAND) {
        true => {
            std::process::exit(0);
        }
        false => {
            handle_command(command, args);
        }
    }
}

fn display_prompt(promt: String) {
    print!("{} ", promt);
    io::stdout().flush().unwrap();
}

fn check_command_in_path(command: &str) -> (bool, String) {
    if let Ok(path) = var("PATH") {
        for p in path.split(':') {
            let (found, fullpath) = check_command_in_folder(command, p);
            if found {
                (true, fullpath);
            }
        }
        (false, String::new())
    } else {
        (false, String::new())
    }
}

fn check_command_in_folder(command: &str, folder: &str) -> (bool, String) {
    let path = Path::new(folder).join(command);

    println!("Checking path: {:?}", path);

    if !path.exists() {
        (false, String::new());
    }

    if let Ok(metadata) = fs::metadata(&path) {
        let permissions = metadata.permissions();
        let mode = permissions.mode();

        // consider the file executable if any execute bit is set
        if mode & EXECUTABLE_PERMISSION != 0 {
            (true, String::from(path.to_str().unwrap()));
        }
    }

    (false, String::new())
}

fn main() {
    loop {
        let prompt = String::from("$");
        display_prompt(prompt);
        handle_inputs();
    }
}
