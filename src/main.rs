// use console::style;
// use std::{io::Write, process::Command};
mod command_handler;
mod infobar;
mod os_info;
mod parser;

fn main() {
    let input = std::io::stdin().lines().next().unwrap().unwrap();
    let parser_ret = parser::parse_input(&input);
    println!("{:?}", parser_ret);
    // loop {
    //     infobar::show_infobar();
    //     std::io::stdout().flush().expect("flush failed!");

    //     let input = std::io::stdin().lines().next().unwrap().unwrap();

    //     let command = input.trim();
    //     let str_cmd = String::from(command);
    //     let parsed_input = parser::parse_input(&str_cmd);
    //     let command = command_handler::build_command(parsed_input);

    //     match command {
    //         Err(_) => {}
    //         Ok(c) => {
    //             let cmd = Command::new(&c.cmd).args(c.args).spawn();
    //             match cmd {
    //                 Err(_) => {
    //                     println!(
    //                         "{}{}",
    //                         style("Unknown command: ").red().bold(),
    //                         String::from(&c.cmd)
    //                     );
    //                 }
    //                 Ok(mut child) => {
    //                     child.wait().expect("AY MIGUEL");
    //                 }
    //             }
    //         }
    //     }
    // }
}
