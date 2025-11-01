#[allow(unused_imports)]
use std::io::{self, Write};

fn handle_command() {
    let mut command = String::new();
    io::stdin().read_line(&mut command).unwrap();
    let command = command.trim();

    match command {
        _ => {
            println!("{}: Command not found", command);
        }
    }
}

fn main() {
    // TODO: Uncomment the code below to pass the first stage
    print!("$ ");
    io::stdout().flush().unwrap();
    handle_command();
}
