use console::style;
use signal_hook::low_level::exit;
use std::{cmp::Ordering, env, path::Path, process::Command};

#[allow(non_camel_case_types)]
enum SpecialCommands {
    cd,
    exit,
    None,
}

pub struct CommandStructure<'a> {
    pub cmd: String,
    pub args: Vec<&'a str>,
}

fn to_special_command(command: &str) -> SpecialCommands {
    if command.cmp("cd") == Ordering::Equal {
        return SpecialCommands::cd;
    }
    if command.cmp("exit") == Ordering::Equal {
        return SpecialCommands::exit;
    }
    return SpecialCommands::None;
}

fn build_command(parsed_commands: Vec<&str>) -> Result<CommandStructure, i16> {
    match to_special_command(parsed_commands[0]) {
        SpecialCommands::cd => {
            cd(parsed_commands);
            return Err(1);
        }
        SpecialCommands::exit => {
            exit(0);
        }
        SpecialCommands::None => {
            return Ok(CommandStructure {
                cmd: parsed_commands[0].to_string(),
                args: parsed_commands[1..].to_vec(),
            });
        }
    }
}

fn cd(parsed_commands: Vec<&str>) {
    let mut root_str = "/home/angelsz";
    if parsed_commands.len() != 1 {
        root_str = parsed_commands[1];
    }
    let root = Path::new(root_str);
    let env_res = env::set_current_dir(&root);
    match env_res {
        Ok(_) => {}
        Err(_) => {
            println!(
                "{}{}",
                style("Unknown directory: ").red(),
                parsed_commands[1]
            )
        }
    }
}

pub fn exec_command(parsed_commands: Vec<&str>) {
    let command = build_command(parsed_commands);
    match command {
        Err(_) => {}
        Ok(c) => {
            let cmd = Command::new(&c.cmd).args(c.args).spawn();
            match cmd {
                Err(_) => {
                    println!(
                        "{}{}",
                        style("Unknown command: ").red(),
                        String::from(&c.cmd)
                    );
                }
                Ok(mut child) => {
                    child.wait().expect("AY MIGUEL");
                }
            }
        }
    }
}
