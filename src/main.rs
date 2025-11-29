use std::env::var;
use std::fs::{self};
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::Command;

const EXIT_COMMAND: &str = "exit";
const EXECUTABLE_PERMISSION: u32 = 0o111;
const BUILTIN_COMMANDS: [&str; 4] = ["echo", "cd", "exit", "type"];

fn _handle_command_not_found(command: &str, context: Option<&str>) {
    println!("{}: {}not found", command, context.unwrap_or(""));
}

fn _handle_command(command: &str, args: Vec<&str>) {
    match command {
        "type" => _type_command(args),
        "echo" => _echo_command(args),
        _ => _execute_command(command, args),
    }
}

fn _execute_command(_command: &str, _args: Vec<&str>) {
    let (is_command_in_path, command_executable) = _check_command_in_path(_command);

    if is_command_in_path {
        let mut command = Command::new(command_executable);
        let command_with_args = command.args(&_args);

        if let Ok(mut command_result) = command_with_args.spawn() {
            command_result.wait().unwrap_or_else(|_| {
                _handle_command_not_found(&_command, Some("command "));
                std::process::exit(1);
            });
        } else {
            _handle_command_not_found(_command, Some("command "));
        }
    } else {
        _handle_command_not_found(_command, Some("command "));
    }
}

fn _type_command(args: Vec<&str>) {
    for arg in &args {
        if BUILTIN_COMMANDS.contains(&arg) {
            println!("{} is a shell builtin", *arg);
        } else if _check_command_in_path(&arg).0 {
            println!("{} is {}", *arg, _check_command_in_path(&arg).1);
        } else {
            _handle_command_not_found(*arg, None);
        }
    }
}

fn _echo_command(args: Vec<&str>) {
    println!("{}", args.join(" "));
}

fn _handle_inputs() {
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
            _handle_command(command, args);
        }
    }
}

fn _display_prompt(promt: String) {
    print!("{} ", promt);
    io::stdout().flush().unwrap();
}

fn _check_command_in_path(command: &str) -> (bool, String) {
    if let Ok(path) = var("PATH") {
        for p in path.split(':') {
            let (found, fullpath) = _check_command_in_folder(command, p);
            if found {
                return (true, fullpath);
            }
        }
    }

    (false, String::new())
}

fn _check_command_in_folder(command: &str, folder: &str) -> (bool, String) {
    let path = Path::new(folder).join(command);

    if !path.exists() {
        return (false, String::new());
    }

    if let Ok(metadata) = fs::metadata(&path) {
        let permissions = metadata.permissions();
        let mode = permissions.mode();

        // consider the file executable if any execute bit is set
        if mode & EXECUTABLE_PERMISSION != 0 {
            return (true, String::from(path.to_str().unwrap()));
        }
    }

    (false, String::new())
}

fn main() {
    loop {
        let prompt = String::from("$");
        _display_prompt(prompt);
        _handle_inputs();
    }
}
