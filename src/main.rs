#[allow(unused_imports)]
use std::io::{self, Write};

fn handle_command(command: String) {
    let mut command_and_args = command.split_whitespace();
    let command = command_and_args.next().unwrap();
    let args: Vec<&str> = command_and_args.collect();

    if command == "echo" {
        println!("{}", args.join(" "));
    } else {
        println!("{}: command not found", command);
    }
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
