#[allow(unused_imports)]
use std::io::{self, Write};

fn handle_command() {
    let mut command = String::new();
    io::stdin().read_line(&mut command).unwrap();
    let command = command.trim();

    const EXIT_COMMAND: &str = "exit";

    match command.contains(EXIT_COMMAND) {
        true => {
            std::process::exit(0);
        }
        false => {
            println!("{}: command not found", command);
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
        handle_command();
    }
}
