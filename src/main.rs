use std::{io::Write, thread};
mod command_handler;
mod infobar;
mod os_info;
mod parser;
mod signal_handler;

fn main() {
    thread::spawn(|| signal_handler::handle());
    loop {
        infobar::show_infobar();
        std::io::stdout().flush().expect("flush failed!");
        let input = std::io::stdin().lines().next().unwrap().unwrap();
        let command = input.trim();
        let str_cmd = String::from(command);
        let parsed_input = parser::parse_input(&str_cmd);
        command_handler::exec_command(parsed_input);
    }
}
