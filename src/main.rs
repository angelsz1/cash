use std::{io::stdin, process::Command};
mod parser;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    // read_line leaves a trailing newline, which trim removes
    let command = input.trim();
    // parser::parse_input(&String::from(command));

    Command::new(command)
        .spawn()
        .expect("Failed to execute command");
}
