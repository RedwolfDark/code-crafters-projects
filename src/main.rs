#[allow(unused_imports)]
use std::io::{self, Write};

fn handle_command_not_found(command: &str) {
    println!("{}: command not found", command);
}

fn handle_command(command: String) {
    let mut command_and_args = command.split_whitespace();
    let command = command_and_args.next().unwrap();
    let args: Vec<&str> = command_and_args.collect();

    let built_in_commands = vec!["echo", "cd", "exit"];

    // handle type command
    if command == "type" {
        for arg in args {
            if built_in_commands.contains(&arg) {
                println!("{} is a built-in command", arg);
            } else {
                handle_command_not_found(arg);
            }
        }
        return;
    }

    // handle built-in commands
    if command == "echo" {
        println!("{}", args.join(" "));
        return;
    }
    handle_command_not_found(command);
}

fn handle_inputs() {
    let mut command = String::new();
    io::stdin().read_line(&mut command).unwrap();
    let command = command.trim().to_string();

    const EXIT_COMMAND: &str = "exit";

    match command.contains(EXIT_COMMAND) {
        true => {
            std::process::exit(0);
        }
        false => {
            handle_command(command);
        }
    }
}

fn display_prompt(promt: String) {
    print!("{} ", promt);
    io::stdout().flush().unwrap();
}

fn main() {
    loop {
        let prompt = String::from("$");
        display_prompt(prompt);
        handle_inputs();
    }
}
